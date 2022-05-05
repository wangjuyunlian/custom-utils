#[cfg(feature = "daemon")]
mod util_daemon;
#[cfg(feature = "logger")]
mod util_log;

#[cfg(feature = "logger")]
pub use log::{debug, error, info, trace};
#[cfg(feature = "logger")]
pub use util_log::{default_debug_logger, default_info_logger, default_logger};

#[cfg(feature = "daemon")]
pub use util_daemon::daemon;
