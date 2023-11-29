#![feature(fs_try_exists)]

mod application;
mod domain;
mod infrastructure;

use application::CliConfig;
use application::SwarmdCommand;
use clap::Parser;
use domain::Env;
use infrastructure::Cfg;

mod package;

use swarmd_instruments::{debug, Instruments};

// TODO: Add man

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let release_name = release_name!();
    let _instruments = Instruments::new(release_name)?;
    debug!("Starting the CLI");

    debug!("Read configuration");
    let cfg = Cfg::from_env()?;

    debug!("Generate Env or Context");
    let env = Env::try_from(cfg)?;

    debug!("Read Arguments");
    let config = CliConfig::parse();

    config.execute(&env).await?;
    Ok(())
}
