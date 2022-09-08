#[cfg(not(feature = "sync"))]
mod async_deamon;
#[cfg(feature = "sync")]
mod sync_deamon;

#[cfg(not(feature = "sync"))]
pub use async_deamon::*;
#[cfg(feature = "sync")]
pub use sync_deamon::*;
