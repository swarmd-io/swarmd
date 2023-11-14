use crate::domain::{auth::AuthContext, worker_config::WorkerConfig, Env};
use anyhow::Context;
use clap::Args;
use console::{style, Emoji};
use instruments::debug;
use tokio::time::Instant;

use super::command::SwarmdCommand;

static DELIVERY: Emoji<'_, '_> = Emoji("🚚 ", "");

const SWARMD_CONFIG_FILE: &str = "swarmd.toml";

#[derive(Debug, Args)]
pub struct DeployArg {
    /// To deploy in production (crazy kid! 🤪)
    #[arg(long, default_value_t = false)]
    prod: bool,
    /// Launch the build command before deploying
    #[arg(long, default_value_t = false)]
    build: bool,
    /*
    /// Upload only, cannot be merged with --prod
    #[arg(long, default_value_t = false)]
    upload_only: bool,
    */
}

#[async_trait::async_trait]
impl SwarmdCommand for DeployArg {
    type Error = anyhow::Error;

    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        debug!("starting deploy execution");
        let auth = AuthContext::auth_cached(env)?;
        env.println(format!("{} {}Deploy...", style("").bold().dim(), DELIVERY))?;

        // Check if we have a proper configuration
        let config =
            WorkerConfig::from_file(SWARMD_CONFIG_FILE).context("Couldn't load swarmd.toml.")?;

        env.println(format!(
            "{} {}Building {}",
            style("").bold().dim(),
            DELIVERY,
            style(&config.name).bold().cyan()
        ))?;
        let now = Instant::now();
        // Ask for a build
        config.execute_build()?;

        let elapsed = now.elapsed();
        env.println(format!(
            "{} {}Built {} in {}",
            style("").bold().dim(),
            DELIVERY,
            style(&config.name).bold().cyan(),
            style(format!("{elapsed:?}")).bold().green()
        ))?;

        let file = config.read_dist_sync()?;

        // Upload Worker and create a project if it doesn't exist
        let project_id = config.create_project_associated(env, &auth).await?;

        let now = Instant::now();
        let worker_id_uploaded = config.upload_worker(env, &auth, &project_id, &file).await?;

        let elapsed = now.elapsed();
        env.println(format!(
            "{} {}Uploaded {} in {}",
            style("").bold().dim(),
            DELIVERY,
            style(&config.name).bold().cyan(),
            style(format!("{elapsed:?}")).bold().green()
        ))?;

        // Deploy worker with config
        let now = Instant::now();
        let _ = config
            .deploy_worker(env, &auth, &project_id, worker_id_uploaded)
            .await?;

        let elapsed = now.elapsed();
        env.println(format!(
            "{} {}Deployed {} in {}",
            style("").bold().dim(),
            DELIVERY,
            style(&config.name).bold().cyan(),
            style(format!("{elapsed:?}")).bold().green()
        ))?;

        Ok(())
    }
}
