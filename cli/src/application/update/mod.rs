use crate::domain::Env;
use clap::Args;
use console::{style, Emoji};

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
        Ok(())
    }
}
