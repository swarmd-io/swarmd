use anyhow::Context;
use tracing_subscriber::{self, layer::SubscriberExt, registry::Registry};

/// Instrument layer for internal traces
#[must_use]
pub struct Instruments {}

impl Instruments {
    /// Create a new `Instruments` stack and register it globally.
    pub fn new() -> anyhow::Result<Self> {
        let subscriber = Registry::default()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer());

        tracing::subscriber::set_global_default(subscriber)
            .with_context(|| "cannot set global default subscriber")?;

        Ok(Self {})
    }
}
