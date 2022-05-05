#[cfg(feature = "logger")]
mod log_util;
#[cfg(feature = "logger")]
pub use log::{debug, error, info, trace};
#[cfg(feature = "logger")]
pub use log_util::{default_debug_logger, default_info_logger, default_logger};
