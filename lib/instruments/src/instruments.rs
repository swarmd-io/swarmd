use std::{borrow::Cow, str::FromStr};

use anyhow::Context;
use sentry::{types::Dsn, ClientInitGuard};
use tracing_subscriber::{self, layer::SubscriberExt, registry::Registry};

/// Instrument layer for internal traces
#[must_use]
pub struct Instruments {
    _client_guard: Option<ClientInitGuard>,
}

impl Instruments {
    /// Create a new `Instruments` stack and register it globally.
    pub fn new(release_name: Option<Cow<'static, str>>) -> anyhow::Result<Self> {
        let guard = sentry::init(sentry::ClientOptions {
            // NOTE(@miaxos): The dsn here is linked to swarmd-cli, if we have another bin, we should move that
            // part to be an argument.
            dsn: Some(Dsn::from_str("https://3ca89d7f605a902ddd39375c2a5b1509@o4506304538345472.ingest.sentry.io/4506304540311552")?),
            debug: false,
            release: release_name,
            // Enable capturing of traces; set this a to lower value in production:
            traces_sample_rate: 1.0,
            ..sentry::ClientOptions::default()
        });

        let subscriber = Registry::default()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(sentry_tracing::layer())
            .with(tracing_subscriber::fmt::layer());

        tracing::subscriber::set_global_default(subscriber)
            .with_context(|| "cannot set global default subscriber")?;

        Ok(Self {
            _client_guard: Some(guard),
        })
    }
}
