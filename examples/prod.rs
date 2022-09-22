use custom_utils::logger::*;
use log::warn;

fn main() {
    logger_stdout_debug();
    warn!("warn");
}
