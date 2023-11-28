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

/// Taken from sentry
#[macro_export]
macro_rules! release_name {
    () => {{
        use std::sync::Once;
        static mut INIT: Once = Once::new();
        static mut RELEASE: Option<String> = None;
        unsafe {
            INIT.call_once(|| {
                RELEASE = option_env!("CARGO_PKG_NAME").and_then(|name| {
                    option_env!("CARGO_PKG_VERSION").map(|version| format!("{}@{}", name, version))
                });
            });
            RELEASE.as_ref().map(|x| {
                let release: &'static str = ::std::mem::transmute(x.as_str());
                ::std::borrow::Cow::Borrowed(release)
            })
        }
    }};
}
