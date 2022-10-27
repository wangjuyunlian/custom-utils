use flexi_logger::writers::LogWriter;
use flexi_logger::{DeferredNow, FormatFunction, Logger};
use log::{debug, info, Record};

pub struct CustomWriter;

impl LogWriter for CustomWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        println!("[{}]", record.args());
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Debug
    }
}

fn main() {
    let mut logger = Logger::try_with_str("info").unwrap();
    logger
        .log_to_writer(Box::new(CustomWriter))
        .start()
        .unwrap();
    info!("info");
    debug!("debug");
}
