use std::time::Duration;

use reqwest::{Client, Url};

use super::{Cfg, Indicator};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Global Environment for the program execution
#[derive(Debug, Clone)]
pub struct Env {
    pub http_client: Client,
    pub http_url: Url, // Login data
    // Stack
    pub indicator: Indicator,
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
