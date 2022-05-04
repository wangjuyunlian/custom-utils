use anyhow::Result;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, FileSpec, FlexiLoggerError, LevelFilter, Logger,
    LoggerHandle, Naming, Record, WriteMode,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use time::format_description::FormatItem;
use time::macros::format_description;

const TS_DASHES_BLANK_COLONS_DOT_BLANK: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");
fn with_thread(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "[{}][{}] {:5} [{}:{}] {}",
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        thread::current().name().unwrap_or("<unnamed>"),
        level.to_string(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
    )
}

pub fn default_logger(app: &str, level: LevelFilter) -> Logger {
    let log_spec = flexi_logger::LogSpecBuilder::new().default(level).build();
    // httplog_2022-04-28_17-39-27_rCURRENT.log
    let a = Logger::with(log_spec)
        .format(with_thread)
        .write_mode(WriteMode::Direct);
    init_target(a, app)
}
#[cfg(feature = "prod")]
fn init_target(mut logger: Logger, app: &str) -> Logger {
    logger.log_to_file(default_file_spec(app)).rotate(
        // 10m
        Criterion::AgeOrSize(Age::Day, 10_000_000),
        Naming::Numbers,
        Cleanup::KeepLogFiles(10),
    )
}

#[cfg(not(feature = "prod"))]
fn init_target(logger: Logger, _app: &str) -> Logger {
    logger.log_to_stdout()
}

#[cfg(feature = "prod")]
const LOG_PATH: &str = "/var/local/log";
#[cfg(not(feature = "prod"))]
const LOG_PATH: &str = "./log";

fn default_file_spec(app: &str) -> FileSpec {
    let path = PathBuf::from_str(LOG_PATH).unwrap().join(app);
    FileSpec::default()
        .directory(path)
        .basename(app)
        .suffix("log")
        // 若为true，则会覆盖rotate种的数字、keep^
        .use_timestamp(false)
}
