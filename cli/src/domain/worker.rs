use anyhow::Context;
use deno_core::v8::IsolateHandle;
use deno_core::FastString;
use deno_core::ModuleSpecifier;
use deno_core::NoopModuleLoader;
use futures::future::select;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::thread::JoinHandle;
use swarmd_local_runtime::worker::{SwarmdWorker, WorkerOptions};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;
use tokio::task::spawn_local;

const INITIAL_SRC_CODE: FastString = FastString::Static(
    r#"
    import { onRequest } from "file:///main.js";

    for await (const conn of Swarmd.fakeHttpSwarmdCreateConnection()) {
        for await (const req of Swarmd.fakeFetchRequest(conn)) {
            const now = new Date(Date.now());
            const url = req.request.url;
            const method = req.request.method;
            console.log(`[LOG] ${now.toISOString()} ${method} - ${url}`);

            try {
                await onRequest({ req });
            } catch (e) {
                console.error(e);
                req.respondWith(new Response(new Blob(["something went wrong"]), {
                  status: 500,
                }));
            }
        }
    }
    "#,
);

#[derive(Debug)]
pub struct WorkerHandle {
    isolate: IsolateHandle,
    tx: Sender<()>,
    worker_over: Receiver<()>,
}

impl WorkerHandle {
    pub async fn terminate_execution(self) -> anyhow::Result<Receiver<()>> {
        self.isolate.terminate_execution();
        // If there is an error, we silently ignore it.
        let _ = self.tx.send(());
        Ok(self.worker_over)
    }
}

pub async fn simple_worker<P: AsRef<Path>>(
    js_path: P,
    tx: Sender<WorkerHandle>,
) -> anyhow::Result<()> {
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

    let isolate = worker.js_runtime.v8_isolate().thread_safe_handle();
    let (tx_stop_worker, rx) = oneshot::channel();
    // To indicate the worker is over
    let (tx_worker_over, rx_worker_over) = oneshot::channel();
    let handle = WorkerHandle {
        isolate,
        tx: tx_stop_worker,
        worker_over: rx_worker_over,
    };
    tx.send(handle).unwrap();

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
    let event_loop_handle = spawn_local(async move {
        let interrupt = Box::pin(async move {
            let result = rx.await;
            result.map_err(|_| anyhow::anyhow!("recv error"))?;
            Ok::<_, anyhow::Error>(())
        });
        let event_loop_fut = Box::pin(worker.js_runtime.run_event_loop2(
            deno_core::PollEventLoopOptions {
                wait_for_inspector: false,
                pump_v8_message_loop: true,
            },
        ));

        match select(interrupt, event_loop_fut).await {
            // If interrupt is over, it means we closed or reloaded the server so we don't need to
            // send back the error.
            futures::future::Either::Left((_, _)) => {
                Ok(())
            }
            futures::future::Either::Right((a, _)) => {
                a
            }
        }
    });
    event_loop_handle.await??;
    // We only return the Error when there is an error which is not due to a `oneshot cancel`
    if let Ok(Err(err)) = evaluate_fut.await {
        return Err(err);
    }
    tx_worker_over
        .send(())
        .map_err(|_| anyhow::anyhow!("Couldn't send worker termination."))?;

    Ok(())
}

pub fn start_background_worker<P: AsRef<Path>>(
    js_path: P,
) -> (JoinHandle<anyhow::Result<()>>, Receiver<WorkerHandle>) {
    let js_path = js_path.as_ref().to_owned();
    let (tx, rx) = oneshot::channel::<WorkerHandle>();
    let handle = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        local.block_on(&runtime, async move {
            if let Err(err) = simple_worker(js_path, tx).await {
                println!("{}", err);
            }
            Ok::<_, anyhow::Error>(())
        })
    });
    (handle, rx)
}
