extern crate core;

#[cfg(feature = "daemon")]
mod util_daemon;
#[cfg(feature = "logger")]
mod util_log;
#[cfg(feature = "timer-util")]
pub mod util_timer;
#[cfg(feature = "tls")]
mod util_tls;
#[cfg(feature = "tls-util")]
mod util_tls_util;

#[cfg(feature = "logger")]
pub use log::{debug, error, info, trace};
#[cfg(feature = "logger")]
pub use util_log::{logger_default, logger_default_debug, logger_default_info};

#[cfg(feature = "daemon")]
pub use util_daemon::daemon;
#[cfg(feature = "tls")]
pub use util_tls::*;
#[cfg(feature = "tls-util")]
pub use util_tls_util::print::*;
#[cfg(feature = "tls-util")]
pub use util_tls_util::*;

#[cfg(feature = "timer-util")]
pub use util_timer::{
    data::{
        Hour::{self, *},
        Minuter::{self, *},
        MonthDay::{self, *},
        Second::{self, *},
        WeekDay::{self, *},
    },
    DayHourMinuterSecondConf, Hours, Minuters, MonthDays, Operator, Seconds, WeekDays,
};
