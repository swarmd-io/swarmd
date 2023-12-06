// Adapted from `deno`.
// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::error::JsError;
use deno_core::located_script_name;
use deno_core::v8;
use deno_core::CompiledWasmModuleStore;
use deno_core::Extension;
use deno_core::FsModuleLoader;
use deno_core::GetErrorClassFn;
use deno_core::JsRuntime;
use deno_core::ModuleCode;
use deno_core::ModuleId;
use deno_core::ModuleLoader;
use deno_core::ModuleSpecifier;
use deno_core::RuntimeOptions;
use deno_core::SharedArrayBufferStore;
use deno_core::Snapshot;
use deno_http::DefaultHttpPropertyExtractor;
use log::debug;
use std::rc::Rc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use crate::permissions::{swarmd_permissions, Permissions};

use crate::js::swarmd_isolate_init;

use crate::bootstrap::BootstrapOptions;
use crate::ops;
use crate::shared::runtime;

use tracing::instrument;

pub type FormatJsErrorFn = dyn Fn(&JsError) -> String + Sync + Send;

#[derive(Clone, Default)]
pub struct ExitCode(Arc<AtomicI32>);

impl ExitCode {
    pub fn get(&self) -> i32 {
        self.0.load(Relaxed)
    }

    pub fn set(&mut self, code: i32) {
        self.0.store(code, Relaxed);
    }
}

/// This worker is created and used by almost all
/// subcommands in Deno executable.
///
/// It provides ops available in the `Deno` namespace.
///
/// All `WebWorker`s created during program execution
/// are descendants of this worker.
pub struct SwarmdWorker {
    pub js_runtime: JsRuntime,
    exit_code: ExitCode,
}

pub struct WorkerOptions {
    pub bootstrap: BootstrapOptions,

    /// JsRuntime extensions, not to be confused with ES modules.
    ///
    /// Extensions register "ops" and JavaScript sources provided in `js` or `esm`
    /// configuration. If you are using a snapshot, then extensions shouldn't
    /// provide JavaScript sources that were already snapshotted.
    pub extensions: Vec<Extension>,

    /// V8 snapshot that should be loaded on startup.
    pub startup_snapshot: Option<Snapshot>,

    /// Optional isolate creation parameters, such as heap limits.
    pub create_params: Option<v8::CreateParams>,

    // Crypto
    pub seed: Option<u64>,

    /// Implementation of `ModuleLoader` which will be
    /// called when V8 requests to load ES modules.
    ///
    /// If not provided runtime will error if code being
    /// executed tries to load modules.
    pub module_loader: Rc<dyn ModuleLoader>,

    /// If Some, print a low-level trace output for ops matching the given patterns.
    pub strace_ops: Option<Vec<String>>,

    /// To format JS error into something else.
    pub format_js_error_fn: Option<Arc<FormatJsErrorFn>>,
    /// Allows to map error type to a string "class" used to represent
    /// error in JavaScript.
    pub get_error_class_fn: Option<GetErrorClassFn>,

    /// The store to use for transferring SharedArrayBuffers between isolates.
    /// If multiple isolates should have the possibility of sharing
    /// SharedArrayBuffers, they should use the same [SharedArrayBufferStore]. If
    /// no [SharedArrayBufferStore] is specified, SharedArrayBuffer can not be
    /// serialized.
    pub shared_array_buffer_store: Option<SharedArrayBufferStore>,

    /// The store to use for transferring `WebAssembly.Module` objects between
    /// isolates.
    /// If multiple isolates should have the possibility of sharing
    /// `WebAssembly.Module` objects, they should use the same
    /// [CompiledWasmModuleStore]. If no [CompiledWasmModuleStore] is specified,
    /// `WebAssembly.Module` objects cannot be serialized.
    pub compiled_wasm_module_store: Option<CompiledWasmModuleStore>,
}

impl Default for WorkerOptions {
    fn default() -> Self {
        Self {
            module_loader: Rc::new(FsModuleLoader),
            seed: None,
            strace_ops: Default::default(),
            compiled_wasm_module_store: Default::default(),
            shared_array_buffer_store: Default::default(),
            format_js_error_fn: Default::default(),
            get_error_class_fn: Default::default(),
            extensions: Default::default(),
            startup_snapshot: Default::default(),
            create_params: Default::default(),
            bootstrap: Default::default(),
        }
    }
}

impl SwarmdWorker {
    #[instrument(skip(main_module, options))]
    pub fn bootstrap_from_options(main_module: ModuleSpecifier, options: WorkerOptions) -> Self {
        let bootstrap_options = options.bootstrap.clone();
        let mut worker = Self::from_options(main_module, options);
        worker.bootstrap(bootstrap_options);
        worker
    }

    #[instrument(skip(main_module, options))]
    pub fn from_options(main_module: ModuleSpecifier, mut options: WorkerOptions) -> Self {
        let exit_code = ExitCode(Arc::new(AtomicI32::new(0)));
        let user_agent = "Swarmd/Edge-Local".to_string();

        // NOTE: ordering is important here, keep it in sync with
        // `runtime/build.rs`!
        let mut extensions: Vec<Extension> = vec![
            swarmd_permissions::init_ops(),
            // Web APIs
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<Permissions>(
                Arc::new(deno_web::BlobStore::default()),
                None,
            ),
            deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(deno_fetch::Options {
                user_agent: user_agent.clone(),
                ..Default::default()
            }),
            deno_websocket::deno_websocket::init_ops_and_esm::<Permissions>(user_agent, None, None),
            deno_crypto::deno_crypto::init_ops_and_esm(None),
            deno_net::deno_net::init_ops_and_esm::<Permissions>(None, None),
            deno_tls::deno_tls::init_ops_and_esm(),
            deno_http::deno_http::init_ops_and_esm::<DefaultHttpPropertyExtractor>(),
            // Ops from this crate
            ops::runtime::swarmd_runtime::init_ops_and_esm(main_module.clone()),
            runtime::init_ops_and_esm(),
            ops::bootstrap::swarmd_bootstrap::init_ops_and_esm(),
            ops::http::swarmd_http_runtime::init_ops_and_esm(),
        ];

        for extension in &mut extensions {
            extension.js_files = std::borrow::Cow::Borrowed(&[]);
            extension.esm_files = std::borrow::Cow::Borrowed(&[]);
            extension.esm_entry_point = None;
        }

        extensions.extend(std::mem::take(&mut options.extensions));

        /*
        *TODO: add it later
        // Clear extension modules from the module map, except preserve `node:*`
        // modules.
        let preserve_snapshotted_modules =
          Some(SUPPORTED_BUILTIN_NODE_MODULES_WITH_PREFIX);
        */

        let js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(options.module_loader.clone()),
            startup_snapshot: options
                .startup_snapshot
                .or_else(|| Some(swarmd_isolate_init())),
            create_params: options.create_params,
            source_map_getter: None,
            skip_op_registration: false,
            get_error_class_fn: options.get_error_class_fn,
            shared_array_buffer_store: options.shared_array_buffer_store.clone(),
            compiled_wasm_module_store: options.compiled_wasm_module_store.clone(),
            extensions,
            preserve_snapshotted_modules: None,
            inspector: false,
            is_main: true,
            feature_checker: None,
            op_metrics_factory_fn: None,
            ..Default::default()
        });

        Self {
            js_runtime,
            exit_code,
        }
    }

    pub fn bootstrap(&mut self, options: BootstrapOptions) {
        self.js_runtime.op_state().borrow_mut().put(options.clone());
        // Bootstrapping stage
        let version = env!("GIT_HASH");
        let target = env!("TARGET");

        // Bootstrapping stage
        let script = format!("globalThis.bootstrapWorker('{}', '{}')", version, target);

        self.js_runtime
            .execute_script(located_script_name!(), ModuleCode::from(script))
            .unwrap();
    }

    /// See [JsRuntime::execute_script](deno_core::JsRuntime::execute_script)
    pub fn execute_script(
        &mut self,
        script_name: &'static str,
        source_code: ModuleCode,
    ) -> Result<v8::Global<v8::Value>, AnyError> {
        self.js_runtime.execute_script(script_name, source_code)
    }

    /// Loads and instantiates specified JavaScript module as "main" module.
    pub async fn preload_main_module(
        &mut self,
        module_specifier: &ModuleSpecifier,
    ) -> Result<ModuleId, AnyError> {
        self.js_runtime
            .load_main_module(module_specifier, None)
            .await
    }

    /// Loads and instantiates specified JavaScript module as "side" module.
    pub async fn preload_side_module(
        &mut self,
        module_specifier: &ModuleSpecifier,
    ) -> Result<ModuleId, AnyError> {
        self.js_runtime
            .load_side_module(module_specifier, None)
            .await
    }

    /// Executes specified JavaScript module.
    pub async fn evaluate_module(&mut self, id: ModuleId) -> Result<(), AnyError> {
        let mut receiver = self.js_runtime.mod_evaluate(id);
        tokio::select! {
          // Not using biased mode leads to non-determinism for relatively simple
          // programs.
          biased;

          maybe_result = &mut receiver => {
            debug!("received module evaluate {:#?}", maybe_result);
            maybe_result.expect("Module evaluation result not provided.")
          }

          event_loop_result = self.run_event_loop(false) => {
            event_loop_result?;
            let maybe_result = receiver.await;
            maybe_result.expect("Module evaluation result not provided.")
          }
        }
    }

    /// Loads, instantiates and executes specified JavaScript module.
    pub async fn execute_side_module(
        &mut self,
        module_specifier: &ModuleSpecifier,
    ) -> Result<(), AnyError> {
        let id = self.preload_side_module(module_specifier).await?;
        self.evaluate_module(id).await
    }

    /// Loads, instantiates and executes specified JavaScript module.
    ///
    /// This module will have "import.meta.main" equal to true.
    pub async fn execute_main_module(
        &mut self,
        module_specifier: &ModuleSpecifier,
    ) -> Result<(), AnyError> {
        let id = self.preload_main_module(module_specifier).await?;
        self.evaluate_module(id).await
    }

    pub async fn run_event_loop(&mut self, wait_for_inspector: bool) -> Result<(), AnyError> {
        self.js_runtime
            .run_event_loop2(deno_core::PollEventLoopOptions {
                wait_for_inspector,
                ..Default::default()
            })
            .await
    }

    /// Return exit code set by the executed code (either in main worker
    /// or one of child web workers).
    pub fn exit_code(&self) -> i32 {
        self.exit_code.get()
    }
}
