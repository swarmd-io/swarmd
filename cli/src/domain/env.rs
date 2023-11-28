use anyhow::Context;
use indicatif::{MultiProgress, ProgressBar};
use reqwest::Url;

use crate::infrastructure::http_client::HttpClient;
use crate::infrastructure::swarmd_client::{SwarmdClient, SWARMD_URL};
use crate::infrastructure::{Cfg, Indicator};

use super::auth::AuthContext;

/// Global Environment for the program execution
#[derive(Debug, Clone)]
pub struct Env {
    pub http_client: HttpClient,
    pub http_url: String, // Login data
    // Stack
    _indicator: Indicator,
}

impl TryFrom<Cfg> for Env {
    type Error = anyhow::Error;

    fn try_from(_value: Cfg) -> Result<Self, Self::Error> {
        Ok(Self {
            http_client: HttpClient::new(),
            http_url: "http://127.0.0.1:8087".to_string(),
            _indicator: Indicator::new(),
        })
    }
}

impl Env {
    /// Print a message into the STDOUT
    pub fn println<I: AsRef<str>>(&self, msg: I) -> std::io::Result<()> {
        println!("{}", msg.as_ref());
        // self.indicator.println(msg)
        Ok(())
    }

    #[allow(dead_code)]
    pub fn follow(&self, progress_bar: ProgressBar) -> ProgressBar {
        self._indicator.follow(progress_bar)
    }

    #[allow(dead_code)]
    pub fn bars(&self) -> &MultiProgress {
        &self._indicator.bars
    }

    /// Get the AuthURL with redirection
    pub fn auth_url_with_local_redirect(&self, port: u16) -> anyhow::Result<Url> {
        format!("{SWARMD_URL}/sign-up?port={port}")
            .parse()
            .context("Can't parse URL properly with this port")
    }

    pub fn swarmd_client(&self) -> anyhow::Result<SwarmdClient> {
        let auth = AuthContext::from_env()?.ok_or(anyhow::anyhow!(
            "You must login with `swarmd login` before."
        ))?;
        let token = auth.token().clone();

        Ok(SwarmdClient::new(
            self.http_url.to_string(),
            &self.http_client,
            token,
        ))
    }
}
