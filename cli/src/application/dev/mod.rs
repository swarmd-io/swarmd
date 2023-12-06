use crate::domain::{
    worker::simple_worker,
    worker_config::{WorkerConfig, SWARMD_CONFIG_FILE},
    Env,
};
use anyhow::Context;
use clap::Args;
use console::{style, Emoji};
use tokio::task::spawn_blocking;

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
            "{} {}The dev server will not reload itself each time you build your project again.",
            style("").bold().dim(),
            DELIVERY
        ))?;

        // Check if we have a proper configuration
        let config =
            WorkerConfig::from_file(SWARMD_CONFIG_FILE).context("Couldn't load swarmd.toml.")?;
        let path_dist = config.path_main_dist().to_path_buf();

        // TODO: Add auto-reload when dist file change.

        let handle = spawn_blocking(|| {
            let handle = std::thread::spawn(|| {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                let local = tokio::task::LocalSet::new();

                local.block_on(&runtime, async move {
                    let mut _worker = simple_worker(path_dist).await?;
                    Ok::<_, anyhow::Error>(())
                })
            });
            handle.join()
        });

        let _ = handle
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?
            .map_err(|_| anyhow::anyhow!("An issue happened while joining the thread"))?
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        Ok(())
    }
}
