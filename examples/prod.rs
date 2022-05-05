use custom_utils::*;

fn main() {
    let _logger = default_debug_logger("prod").unwrap();
    debug!("abc");
    info!("abc");
}
