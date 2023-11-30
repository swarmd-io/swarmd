use std::env::current_dir;

use crate::domain::Env;
use clap::Args;
use console::{style, Emoji};
use self_update::get_target;
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
        let status = spawn_blocking(|| {
            let release = self_update::backends::github::Update::configure()
                .repo_owner("swarmd-io")
                .repo_name("swarmd")
                .bin_name("swarmd")
                .current_version(self_update::cargo_crate_version!())
                .build()?
                .get_latest_release()?;

            let last_version = release.version;
            let target = get_target();
            let bin_path = format!("swarmd-v{last_version}-{target}/swarmd");

            let status = self_update::backends::github::Update::configure()
                .repo_owner("swarmd-io")
                .repo_name("swarmd")
                .bin_path_in_archive(&bin_path)
                .bin_name("swarmd")
                .show_download_progress(false)
                .current_version(self_update::cargo_crate_version!())
                .show_output(false)
                .no_confirm(true)
                .bin_install_path(current_dir().unwrap())
                .build()?
                .update()?;

            Ok::<_, anyhow::Error>(status)
        })
        .await??;

        env.println(format!(
            "{} {}New version `{}` installed",
            style("").bold().dim(),
            DELIVERY,
            style(status.version()).bold().cyan(),
        ))?;

        Ok(())
    }
}
