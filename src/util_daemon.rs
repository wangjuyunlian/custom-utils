#[cfg(feature = "daemon-async")]
mod async_deamon;
#[cfg(feature = "daemon-sync")]
mod sync_deamon;

#[cfg(feature = "daemon-async")]
pub use async_deamon::*;
#[cfg(feature = "daemon-sync")]
pub use sync_deamon::*;
