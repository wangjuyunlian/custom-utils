use custom_utils::logger::*;

fn main() {
    let _logger = logger_stdout_debug();
    debug!("abc");
    info!("abc");
}
