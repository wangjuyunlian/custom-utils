use custom_utils::*;

fn main() {
    let _logger = logger_default_debug("prod").unwrap();
    debug!("abc");
    info!("abc");
}
