use deno_core::FastString;

/// We ensure that a basic function can be executed but we do not check the result
#[test]
fn should_work_to_execute_basic_function() {
    use deno_core::{FsModuleLoader, ModuleSpecifier};
    use std::rc::Rc;
    use swarmd_local_runtime::worker::{SwarmdWorker, WorkerOptions};

    let js_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/basic/hello.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let mut worker = SwarmdWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            extensions: vec![],
            ..Default::default()
        },
    );

    let result = worker.execute_script(
        "file:///test.js",
        FastString::Static(
            r#"
      (() => {
          console.log("Simple log from Worker");
      })()
      "#,
        ),
    );

    assert!(result.is_ok());
}

/// We ensure that a basic function can throw
#[test]
fn throwing_error_basic() {
    use deno_core::{FsModuleLoader, ModuleSpecifier};
    use std::rc::Rc;
    use swarmd_local_runtime::worker::{SwarmdWorker, WorkerOptions};

    let js_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/basic/hello.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let mut worker = SwarmdWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            extensions: vec![],
            ..Default::default()
        },
    );

    let result = worker.execute_script(
        "file:///test.js",
        FastString::Static(
            r#"
      (() => {
          odzekdiozedjoizej
          return 1;
      })()
      "#,
        ),
    );

    assert!(result.is_err());

    let err = result.unwrap_err();
    insta::assert_display_snapshot!(err);
}
