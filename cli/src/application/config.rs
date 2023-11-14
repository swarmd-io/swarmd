use crate::domain::Env;
use crate::infrastructure::{NAME, VERSION};
use async_trait::async_trait;
use clap::{Parser, Subcommand};

use super::{command::SwarmdCommand, create::CreateArg, deploy::DeployArg, login::LoginArg};

#[derive(Parser, Debug)]
#[command(name = NAME)]
#[command(author = "Anthony G. <anthony@swarmd.io>")]
#[command(version = VERSION)]
#[command(about = "Swarmd CLI allow you to interact with the Swarmd ecosystem to deploy workers through the Swarmd's EdgeNetwork.", long_about = None)]
pub struct CliConfig {
    #[command(subcommand)]
    command: Commands,
}

#[async_trait]
impl SwarmdCommand for CliConfig {
    type Error = anyhow::Error;
    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        self.command.execute(env).await
    }
}

/// List of commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login to Swarmd Edge network.
    Login(LoginArg),
    /// Create a new project
    Create(CreateArg),
    /// Deploy a Worker
    Deploy(DeployArg),
}

#[async_trait]
impl SwarmdCommand for Commands {
    type Error = anyhow::Error;
    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        match self {
            Commands::Login(arg) => arg.execute(env).await,
            Commands::Create(arg) => arg.execute(env).await,
            Commands::Deploy(arg) => arg.execute(env).await,
        }
    }
}
