//! Login part of Swarmd CLI
//!
//! We used short lived token for every operation on Swarmd, but when working on the CLI it's usual
//! that you do a lot of shit everyday and some days not at all.
//!
//! So, we generate a 7 day long token for the usual login flow which will allow you to:
//!
//! - Create projects
//! - Deploy projects
use std::time::Duration;

use anyhow::Context;
use clap::Args;
use console::{style, Emoji};
use indicatif::ProgressBar;
use instruments::debug;

use crate::{
    domain::{auth::AuthContext, Env},
    package::HttpAuthServer,
};

use super::command::SwarmdCommand;

static LOCK: Emoji<'_, '_> = Emoji("🔒 ", "");

#[derive(Debug, Args)]
pub struct LoginArg {
    /// Number of milliseconds we wait for the authentification flow.
    #[arg(short, long, value_name = "Seconds", default_value_t = 120)]
    timeout: u64,
}

#[async_trait::async_trait]
impl SwarmdCommand for LoginArg {
    type Error = anyhow::Error;

    async fn execute(&self, env: &Env) -> anyhow::Result<()> {
        debug!("starting login execution");

        let timeout = Duration::from_secs(self.timeout);

        env.println(format!("{} {}Login in...", style("").bold().dim(), LOCK))?;

        let pb = ProgressBar::new_spinner()
            .with_message("Waiting for authentication to be completed...");
        pb.enable_steady_tick(Duration::from_millis(100));

        // TODO(@miaxos): What happens when the port 3001 is taken? Failed.
        // Should fix this by taking a random port.

        let url = env.auth_url_with_local_redirect(3001)?;
        let server = HttpAuthServer::new(3001);

        let _ = webbrowser::open(url.as_str());
        let get_token_fut = server.get_token(timeout);

        let token = get_token_fut.await.context("Couldn't get token")?;

        let auth_ctx = AuthContext::new_from_token(token)?;
        auth_ctx.save().context("Couldn't save the auth")?;

        Ok(())
    }
}
