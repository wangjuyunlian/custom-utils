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

    custom_utils::logger::logger_feature(Debug)
        .module("custom_utils", Debug)
        .build("my-app");

    custom_utils::logger::custom_build(Debug)
        .module("custom_utils", Debug)
        .build_default()
        .log_to_stdout()
        ._start();
}
