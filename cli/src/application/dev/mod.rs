use std::{env::current_dir, time::Duration};

use crate::domain::{
    worker::start_background_worker,
    worker_config::{WorkerConfig, SWARMD_CONFIG_FILE},
    Env,
};
use anyhow::Context;
use clap::Args;
use console::{style, Emoji};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

use super::SwarmdCommand;
use swarmd_instruments::debug;

static DELIVERY: Emoji<'_, '_> = Emoji("🚚 ", "");

#[derive(Debug, Args)]
pub struct DevArg {}

#[async_trait::async_trait]
impl SwarmdCommand for DevArg {
    type Error = anyhow::Error;

    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        debug!("starting dev");
        env.println(format!(
            "{} {}Dev server starting...",
            style("").bold().dim(),
            DELIVERY
        ))?;
        env.println(format!(
            "{} {}The dev server will not reload itself each time you build your project again...",
            style("").bold().dim(),
            DELIVERY
        ))?;
        env.println(format!(
            "{} {}Feel free to CTRL-C to stop the actual server...",
            style("").bold().dim(),
            DELIVERY
        ))?;

        // Check if we have a proper configuration
        let config =
            WorkerConfig::from_file(SWARMD_CONFIG_FILE).context("Couldn't load swarmd.toml.")?;
        let path_dist = config.path_main_dist().to_path_buf();

        let (notify_sender, mut notify_receiver) = tokio::sync::mpsc::channel(1024);

        let path = current_dir()?;
        let mut debouncer = new_debouncer(Duration::from_secs(1), None, move |evt| {
            notify_sender.blocking_send(evt).unwrap();
        })?;

        let src_path = path.join("src");
        let node_modules = path.join("node_modules");
        debouncer
            .watcher()
            .watch(&src_path, RecursiveMode::Recursive)?;
        debouncer
            .watcher()
            .watch(&node_modules, RecursiveMode::Recursive)?;

        debouncer
            .cache()
            .add_root(&src_path, RecursiveMode::Recursive);
        debouncer
            .cache()
            .add_root(&node_modules, RecursiveMode::Recursive);

        let env_cloned = env.clone();
        let handle = tokio::spawn(async move {
            config.execute_no_log()?;
            let mut handle = Some(start_background_worker(path_dist.clone()));

            env_cloned.println("")?;
            env_cloned.println("")?;

            env_cloned.println(format!(
                "Worker available at: {}",
                style("http://127.0.0.1:13337").cyan().bold().dim(),
            ))?;

            while let Some(elt) = notify_receiver.recv().await {
                let mut should_reload = false;
                match elt {
                    Ok(_) => {
                        should_reload = true;
                    }
                    Err(errors) => {
                        for e in errors {
                            let kind = e.kind;
                            env_cloned
                                .println(format!("{}:", style(format!("{kind:?}")).red().bold()))?;

                            for path in e.paths {
                                env_cloned.println(format!(
                                    "  [{}]",
                                    style(path.to_str().unwrap_or("")).italic()
                                ))?;
                            }
                        }
                    }
                }

                if should_reload {
                    if let Some((_, isolate_handle)) = handle.take() {
                        let isolate = isolate_handle.await.expect(
                        "Shouldn't fail as the isolate is send directly after the isolate creation",
                    );
                        let worker_over = isolate.terminate_execution().await?;
                        let _ = worker_over.await;
                    }
                    env_cloned.println(format!("{}", style("Reloading...").cyan().bold()))?;
                    config.execute_build()?;
                    env_cloned.println(format!("{}", style("Reloaded").cyan().bold()))?;
                    handle = Some(start_background_worker(path_dist.clone()));
                }
            }
            Ok::<_, anyhow::Error>(())
        });

        let _ = handle
            .await
            .map_err(|_| anyhow::anyhow!("An issue happened while joining the thread"))?
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        Ok(())
    }
}
