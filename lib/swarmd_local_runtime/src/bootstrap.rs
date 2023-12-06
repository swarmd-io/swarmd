// Adapted from `deno`.
// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
use deno_core::ModuleSpecifier;

/// The log level to use when printing diagnostic log messages, warnings,
/// or errors in the worker.
///
/// Note: This is disconnected with the log crate's log level and the Rust code
/// in this crate will respect that value instead. To specify that, use
/// `log::set_max_level`.
#[derive(Debug, Default, Clone, Copy)]
pub enum WorkerLogLevel {
    // WARNING: Ensure this is kept in sync with
    // the JS values (search for LogLevel).
    Error = 1,
    Warn = 2,
    #[default]
    Info = 3,
    Debug = 4,
}

impl From<log::Level> for WorkerLogLevel {
    fn from(value: log::Level) -> Self {
        match value {
            log::Level::Error => WorkerLogLevel::Error,
            log::Level::Warn => WorkerLogLevel::Warn,
            log::Level::Info => WorkerLogLevel::Info,
            log::Level::Debug => WorkerLogLevel::Debug,
            log::Level::Trace => WorkerLogLevel::Debug,
        }
    }
}

/// Common bootstrap options for MainWorker & WebWorker
#[derive(Clone)]
pub struct BootstrapOptions {
    pub cpu_count: usize,
    pub log_level: WorkerLogLevel,
    pub locale: String,
    pub location: Option<ModuleSpecifier>,
    pub user_agent: String,
}

impl Default for BootstrapOptions {
    fn default() -> Self {
        // We let the cpu count to 1 for now.
        let cpu_count = 1;
        let runtime_version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("Swarmd/Local-{runtime_version}");

        Self {
            user_agent,
            cpu_count,
            log_level: Default::default(),
            locale: "en".to_string(),
            location: Default::default(),
        }
    }
}
