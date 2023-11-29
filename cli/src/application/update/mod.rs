use crate::domain::Env;
use clap::Args;
use console::{style, Emoji};
use tokio::task::spawn_blocking;

use super::SwarmdCommand;
use swarmd_instruments::debug;

static DELIVERY: Emoji<'_, '_> = Emoji("🚚 ", "");

#[derive(Debug, Args)]
pub struct UpdateArg {}

#[async_trait::async_trait]
impl SwarmdCommand for UpdateArg {
    type Error = anyhow::Error;

    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        debug!("starting updating");
        env.println(format!(
            "{} {}Updating...",
            style("").bold().dim(),
            DELIVERY
        ))?;
        let _ = spawn_blocking(|| {
            let status = self_update::backends::github::Update::configure()
                .repo_owner("swarmd-io")
                .repo_name("swarmd")
                .bin_name("swarmd")
                .show_download_progress(true)
                .current_version("swarmd-v.0.1.9")
                .build()?
                .update()?;

            println!("Update status: `{:?}`!", status);
            Ok::<_, anyhow::Error>(())
        })
        .await;

        Ok(())
    }
}
