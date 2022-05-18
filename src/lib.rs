extern crate core;

#[cfg(feature = "daemon")]
mod util_daemon;
#[cfg(feature = "logger")]
mod util_log;
#[cfg(feature = "tls")]
mod util_tls;
#[cfg(feature = "tls-util")]
mod util_tls_util;

#[cfg(feature = "logger")]
pub mod logger {
    pub use crate::util_log::{logger_default, logger_default_debug, logger_default_info};
    pub use log::{debug, error, info, trace};
}

#[cfg(feature = "daemon")]
pub mod daemon {
    pub use crate::util_daemon::daemon;
}

#[cfg(feature = "tls")]
pub mod tls {
    pub use crate::util_tls::*;
}

#[cfg(feature = "tls-util")]
pub mod tls_util {
    pub use crate::util_tls_util::print::*;
    pub use crate::util_tls_util::*;
}

#[cfg(feature = "timer")]
pub mod timer {
    pub use timer_util::*;
}
