use config::Config;
use serde::{Deserialize, Serialize};

/// Configuration file for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cfg {}

impl Cfg {
    /// Read the associated configuration env
    pub fn from_env() -> anyhow::Result<Cfg> {
        let settings = Config::builder()
            .add_source(
                config::Environment::with_prefix("SWARMD")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()?
            .try_deserialize::<Cfg>()?;

        Ok(settings)
    }
}
