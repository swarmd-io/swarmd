use std::time::Duration;

use crate::{
    domain::auth::AuthContext,
    infrastructure::{NAME, VERSION},
};
use reqwest::{Client, ClientBuilder};

use swarmd_generated::apis::configuration::{ApiKey, Configuration as SwarmdConfiguration};

use super::http_client::HttpClient;

/// Cheap to clone
#[derive(Debug, Clone)]
pub struct SwarmdClient {
    config: SwarmdConfiguration,
}

impl SwarmdClient {
    pub fn new(base: String, http_client: &HttpClient, token: String) -> Self {
        let mut config = SwarmdConfiguration::new();

        config.user_agent = Some(format!("{NAME}-{VERSION}"));
        config.client = http_client.as_ref().clone();
        config.base_path = base;
        config.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: token,
        });

        Self { config }
    }
}

impl AsRef<SwarmdConfiguration> for SwarmdClient {
    fn as_ref(&self) -> &SwarmdConfiguration {
        &self.config
    }
}
