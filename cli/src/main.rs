mod application;
mod domain;
mod infrastructure;

use std::time::Duration;

use application::CliConfig;
use application::SwarmdCommand;
use clap::Parser;
use console::style;
use domain::Env;
use infrastructure::{updater::check_if_update_available, Cfg};

mod package;

use swarmd_instruments::{debug, Instruments};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let release_name = release_name!();
    let _instruments = Instruments::new(release_name)?;
    debug!("Starting the CLI");

    let last_release = tokio::spawn(check_if_update_available());

    debug!("Read configuration");
    let cfg = Cfg::from_env()?;

    debug!("Generate Env or Context");
    let env = Env::try_from(cfg)?;

    // TODO(@miaxos): add a caching mechanism to avoid fetching the version each time.
    let _ = tokio::time::timeout(Duration::from_secs(1), async move {
        if let Some(release) = last_release.await?? {
            let line = format!(
                "{} `{}`",
                style("Current version").yellow().dim(),
                style(self_update::cargo_crate_version!()).bold().cyan(),
            );
            let line2 = format!(
                "{} `{}` {}",
                style("New version").yellow().dim(),
                style(release).bold().cyan(),
                style("available").bold().yellow().dim(),
            );
            let line3 = format!(
                "{} `{}`",
                style("Please update by doing").yellow().dim(),
                style("swarmd update").bold().magenta(),
            );
            println!("-----------------");
            println!("{}", line);
            println!("{}", line2);
            println!("{}", line3);
            println!("-----------------");
        }

        Ok::<_, anyhow::Error>(())
    })
    .await;

    debug!("Read Arguments");
    let config = CliConfig::parse();

    config.execute(&env).await?;

    Ok(())
}
