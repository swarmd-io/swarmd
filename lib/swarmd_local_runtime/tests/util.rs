use deno_core::{FsModuleLoader, ModuleSpecifier};
use std::rc::Rc;
use swarmd_local_runtime::worker::{SwarmdWorker, WorkerOptions};

pub fn simple_worker(js_path: &str) -> (SwarmdWorker, ModuleSpecifier) {
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let worker = SwarmdWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            extensions: vec![],
            ..Default::default()
        },
    );

    (worker, main_module)
}
