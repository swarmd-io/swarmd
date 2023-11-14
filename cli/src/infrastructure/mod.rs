mod config;
pub use config::Cfg;

mod indicator;
pub use indicator::Indicator;

pub mod fs;

pub mod http_client;

pub mod swarmd_client;

pub const NAME: &str = env!("CARGO_CRATE_NAME");
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("GIT_HASH"),
    "-",
    env!("BUILD_DATE")
);