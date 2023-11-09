use std::time::Duration;

use anyhow::Context;
use indicatif::{MultiProgress, ProgressBar};
use reqwest::{Client, Url};

use crate::infrastructure::{Cfg, Indicator};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Global Environment for the program execution
#[derive(Debug, Clone)]
pub struct Env {
    pub http_client: Client,
    pub http_url: Url, // Login data
    // Stack
    indicator: Indicator,
}

impl TryFrom<Cfg> for Env {
    type Error = anyhow::Error;

    fn try_from(_value: Cfg) -> Result<Self, Self::Error> {
        Ok(Self {
            http_client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(60))
                .gzip(true)
                .user_agent(USER_AGENT)
                .build()?,
            http_url: "http://127.0.0.1:3000".parse()?,
            indicator: Indicator::new(),
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

    pub fn follow(&self, progress_bar: ProgressBar) -> ProgressBar {
        self.indicator.follow(progress_bar)
    }

    pub fn bars(&self) -> &MultiProgress {
        &self.indicator.bars
    }

    /// Get the AuthURL with redirection
    pub fn auth_url_with_local_redirect(&self, port: u16) -> anyhow::Result<Url> {
        format!("http://localhost:3000/sign-up?port={port}")
            .parse()
            .context("Can't parse URL properly with this port")
    }
}
