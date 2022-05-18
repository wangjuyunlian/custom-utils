use custom_utils::logger::logger_default_debug;
use custom_utils::*;
use log::{debug, info};

#[tokio::main]
async fn main() {
    let _logger = logger_default_debug("dev").unwrap();
    debug!("abc");
    info!("abc");
    let handle = daemon();
    if let Err(_e) = handle.await {}
}
