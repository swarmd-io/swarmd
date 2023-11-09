use std::{path::PathBuf, time::Duration};

use anyhow::{bail, Context};
use clap::{Args, ValueEnum};
use console::{style, Emoji};
use git2::Repository;
use indicatif::ProgressBar;
use instruments::debug;

use crate::{
    domain::{auth::AuthContext, Env},
    package::HttpAuthServer,
};

use super::command::SwarmdCommand;

static CREATE: Emoji<'_, '_> = Emoji("📦 ", "");
static SAVE: Emoji<'_, '_> = Emoji("💾 ", "");
static OK: Emoji<'_, '_> = Emoji("✅ ", "");

const TYPESCRIPT_TEMPLATE: &str = "https://github.com/Miaxos/github-control.git";

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum TemplatePossible {
    Typescript,
}

#[derive(Debug, Args)]
pub struct CreateArg {
    /// Create a new Swarmd Worker project.
    #[arg(required = true, value_name = "name")]
    name: String,
    #[arg(long, short, value_name = "template", value_enum, default_value_t = TemplatePossible::Typescript)]
    template: TemplatePossible,
}

#[async_trait::async_trait]
impl SwarmdCommand for CreateArg {
    type Error = anyhow::Error;

    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        debug!("start creation");

        env.println(format!(
            "{} {}Creating {}...",
            style("").bold().dim(),
            CREATE,
            style(&self.name).cyan().bold(),
        ))?;

        let mut base = std::env::current_dir().context("Couldn't read current directory")?;
        base.push(&self.name);

        if let Ok(true) = std::fs::try_exists(&base) {
            bail!("A file or folder already exist at the given path.");
        }

        env.println(format!(
            "{} {}Connecting to {} and Cloning {}...",
            style("").bold().dim(),
            SAVE,
            style("Github").green().bold(),
            style("swarmd-io/typescript-template").magenta(),
        ))?;

        let repo = Repository::clone(TYPESCRIPT_TEMPLATE, &base)
            .context("Couldn't clone the templated repository")?;
        drop(repo);

        let mut git_hidden_repository = base.clone();
        git_hidden_repository.push(".git");
        std::fs::remove_dir_all(git_hidden_repository)?;

        // TODO: Create the config file

        env.println(format!(
            "{} {}{} has been {}",
            style("").bold().dim(),
            OK,
            style(&self.name).cyan().bold(),
            style("created").magenta().bold(),
        ))?;

        env.println("")?;
        env.println("")?;

        env.println(format!(
            "{}You can now modify and deploy your first {} by going inside {} and running {}.",
            style("").bold().dim(),
            style("Swarmd Worker").magenta().bold(),
            style(&self.name).cyan().bold(),
            style("swarmd deploy --new").magenta().bold(),
        ))?;
        Ok(())
    }
}
