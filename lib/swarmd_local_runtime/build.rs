// Adapted from `deno`.
// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

mod shared {
    include!("src/shared.rs");
}

mod permissions {
    include!("src/permissions/mod.rs");
}

mod startup_snapshot {
    use super::*;
    use deno_core::op2;
    use deno_core::snapshot_util::*;
    use deno_core::Extension;
    use deno_core::OpState;
    use deno_http::DefaultHttpPropertyExtractor;
    use permissions::swarmd_permissions;
    use shared::maybe_transpile_source;
    use shared::runtime;

    // Keep in sync with `runtime/ops/bootstrap.rs`
    #[derive(serde::Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct SnapshotOptions {
        pub swarmd_version: String,
        pub ts_version: String,
        pub v8_version: &'static str,
        pub target: String,
    }

    // TODO(@littledivy): Remove this once we get rid of deno_runtime snapshots.
    #[op2]
    #[serde]
    pub fn op_snapshot_options(_: &mut OpState) -> SnapshotOptions {
        SnapshotOptions::default()
    }

    deno_core::extension!(snapshot, ops = [op_snapshot_options]);

    #[derive(Clone)]
    struct Permissions;

    impl deno_fetch::FetchPermissions for Permissions {
        fn check_net_url(
            &mut self,
            _url: &deno_core::url::Url,
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }

        fn check_read(
            &mut self,
            _p: &Path,
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }
    }

    impl deno_websocket::WebSocketPermissions for Permissions {
        fn check_net_url(
            &mut self,
            _url: &deno_core::url::Url,
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }
    }

    impl deno_web::TimersPermission for Permissions {
        fn allow_hrtime(&mut self) -> bool {
            unreachable!("snapshotting!")
        }
    }

    impl deno_net::NetPermissions for Permissions {
        fn check_net<T: AsRef<str>>(
            &mut self,
            _host: &(T, Option<u16>),
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }

        fn check_read(
            &mut self,
            _p: &Path,
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }

        fn check_write(
            &mut self,
            _p: &Path,
            _api_name: &str,
        ) -> Result<(), deno_core::error::AnyError> {
            unreachable!("snapshotting!")
        }
    }

    pub fn create_runtime_snapshot(snapshot_path: PathBuf) {
        // NOTE: ordering is important here, keep it in sync with
        // `runtime/worker.rs`!
        let mut extensions: Vec<Extension> = vec![
            swarmd_permissions::init_ops(),
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<Permissions>(
                Default::default(),
                Default::default(),
            ),
            deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(Default::default()),
            deno_websocket::deno_websocket::init_ops_and_esm::<Permissions>(
                "".to_owned(),
                None,
                None,
            ),
            deno_crypto::deno_crypto::init_ops_and_esm(None),
            deno_net::deno_net::init_ops_and_esm::<Permissions>(None, None),
            deno_tls::deno_tls::init_ops_and_esm(),
            deno_http::deno_http::init_ops_and_esm::<DefaultHttpPropertyExtractor>(),
            runtime::init_ops_and_esm(),
            snapshot::init_ops_and_esm(),
        ];

        for extension in &mut extensions {
            for source in extension.esm_files.to_mut() {
                maybe_transpile_source(source).unwrap();
            }
            for source in extension.js_files.to_mut() {
                maybe_transpile_source(source).unwrap();
            }
        }

        let output = create_snapshot(CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            snapshot_path,
            startup_snapshot: None,
            extensions,
            compression_cb: None,
            with_runtime_cb: None,
            skip_op_registration: false,
        });

        for path in output.files_loaded_during_snapshot {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
    let o = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Main snapshot
    let runtime_snapshot_path = o.join("RUNTIME_SNAPSHOT.bin");

    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let date = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .unwrap();
    let date = String::from_utf8(date.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    startup_snapshot::create_runtime_snapshot(runtime_snapshot_path)
}
