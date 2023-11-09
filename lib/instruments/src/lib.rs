mod instruments;
pub use instruments::Instruments;

pub use tracing::{debug, error, info, info_span, instrument, warn};

pub mod reexport {
    pub use tracing;
}
