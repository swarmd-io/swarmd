use std::time::Duration;

use crate::infrastructure::{NAME, VERSION};
use reqwest::{Client, ClientBuilder};

/// Cheap to clone
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: ClientBuilder::new()
                .user_agent(format!("{NAME}-{VERSION}"))
                .https_only(false) // Should be true on prod
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Couldn't create HttpClient."),
        }
    }
}

impl AsRef<Client> for HttpClient {
    fn as_ref(&self) -> &Client {
        &self.client
    }
}
