#![feature(result_option_inspect)]
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

use instruments::{debug, Instruments};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _instruments = Instruments::new()?;
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
