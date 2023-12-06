use anyhow::Context;
use deno_core::FastString;
use deno_core::ModuleSpecifier;
use deno_core::NoopModuleLoader;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use swarmd_local_runtime::worker::{SwarmdWorker, WorkerOptions};

const INITIAL_SRC_CODE: FastString = FastString::Static(
    r#"
    import { onRequest } from "file:///main.js";

    for await (const conn of Swarmd.fakeHttpSwarmdCreateConnection()) {
        for await (const req of Swarmd.fakeFetchRequest(conn)) {
            const now = new Date(Date.now());
            const url = req.request.url;
            const method = req.request.method;
            console.log(`[LOG] ${now.toISOString()} ${method} - ${url}`);

            await onRequest({ req });
        }
    }
    "#,
);

pub async fn simple_worker<P: AsRef<Path>>(js_path: P) -> anyhow::Result<SwarmdWorker> {
    let main_module = ModuleSpecifier::from_str("file://fakemodule.js")?;

    let mut worker = SwarmdWorker::bootstrap_from_options(
        main_module,
        WorkerOptions {
            module_loader: Rc::new(NoopModuleLoader),
            extensions: vec![],
            ..Default::default()
        },
    );

    let src_code = std::fs::read_to_string(js_path).context("Coulnd't read the dist file")?;
    let fake_main = ModuleSpecifier::from_str("file:///main.js")?;

    let _ = worker
        .js_runtime
        .load_side_module(
            &fake_main,
            Some(FastString::Owned(src_code.into_boxed_str())),
        )
        .await?;

    let main = ModuleSpecifier::from_str("file:///swarmd.js")?;

    let id = worker
        .js_runtime
        .load_main_module(&main, Some(INITIAL_SRC_CODE))
        .await?;

    let evaluate_fut = worker.js_runtime.mod_evaluate(id);
    worker.run_event_loop(false).await?;
    evaluate_fut.await??;

    Ok(worker)
}
