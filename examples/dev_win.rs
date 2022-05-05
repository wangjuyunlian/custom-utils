use custom_utils::*;

#[tokio::main]
async fn main() {
    let _logger = default_debug_logger("dev").unwrap();
    debug!("abc");
    info!("abc");
    let handle = daemon();
    if let Err(_e) = handle.await {}
}
