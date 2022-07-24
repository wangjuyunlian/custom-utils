use custom_utils::logger::*;
use log::warn;

fn main() {
    let _logger = logger_stdout_debug();
    debug!("abc");
    info!("abc");
    warn!("warn");
    error!("error");
}
