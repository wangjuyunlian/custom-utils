use crate::daemon::daemon;
use custom_utils::*;
use log::LevelFilter::{Debug, Info};
use log::{debug, info};

#[tokio::main]
async fn main() {
    let handle = daemon();
    if let Err(_e) = handle.await {}

    custom_utils::logger::logger_feature("dev", Debug, Info)
        .module("custom_utils", Debug)
        .build();

    custom_utils::logger::custom_build(Debug)
        .module("custom_utils", Debug)
        .build_default()
        .log_to_stdout()
        ._start();

    debug!("abc");
    info!("abc");
}
