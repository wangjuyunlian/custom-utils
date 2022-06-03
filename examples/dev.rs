use crate::daemon::daemon;
use custom_utils::*;
use log::LevelFilter::Debug;
use log::{debug, info};

#[tokio::main]
async fn main() {
    debug!("abc");
    info!("abc");
    let handle = daemon();
    if let Err(_e) = handle.await {}

    custom_utils::logger::LoggerBuilder::default(Debug)
        .build_default()
        .log_to_stdout()
        .start();
}
