use anyhow::Result;
use flexi_logger::Age;
use flexi_logger::{Cleanup, Criterion, FileSpec, Naming};
use flexi_logger::{
    DeferredNow, FormatFunction, LevelFilter, LogSpecBuilder, Logger, LoggerHandle, Record,
    WriteMode,
};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
// #[cfg(feature = "prod")]
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

pub struct LoggerBuilder {
    log_spec_builder: LogSpecBuilder,
}
impl LoggerBuilder {
    pub fn default(level: LevelFilter) -> Self {
        let mut log_spec_builder = LogSpecBuilder::new();
        log_spec_builder.default(level);
        Self { log_spec_builder }
    }
    pub fn module<M: AsRef<str>>(mut self, module_name: M, lf: LevelFilter) -> Self {
        self.log_spec_builder.module(module_name, lf);
        self
    }
    // pub fn new(log_spec_builder: LogSpecBuilder) -> Self {
    //     Self { log_spec_builder }
    // }
    pub fn build_default(self) -> LoggerBuilder2 {
        LoggerBuilder2 {
            logger: Logger::with(self.log_spec_builder.build())
                .format(with_thread)
                .write_mode(WriteMode::Direct),
        }
    }
    pub fn build_with(self, format: FormatFunction, write_mode: WriteMode) -> LoggerBuilder2 {
        LoggerBuilder2 {
            logger: Logger::with(self.log_spec_builder.build())
                .format(format)
                .write_mode(write_mode),
        }
    }
}
pub struct LoggerBuilder2 {
    logger: Logger,
}
pub struct LoggerBuilder3 {
    logger: Logger,
}
impl LoggerBuilder3 {
    pub fn start(self) -> LoggerHandle {
        self.logger.start().unwrap()
    }
    pub fn start_with_specfile(self, p: impl AsRef<Path>) -> LoggerHandle {
        self.logger.start_with_specfile(p).unwrap()
    }
    pub fn start_with_specfile_default(self, app: &str) -> LoggerHandle {
        let path = PathBuf::from_str("/var/local/etc/")
            .unwrap()
            .join(app)
            .join("logspecification.toml");
        self.logger.start_with_specfile(path).unwrap()
    }
}
impl LoggerBuilder2 {
    pub fn log_to_stdout(self) -> LoggerBuilder3 {
        LoggerBuilder3 {
            logger: self.logger.log_to_stdout(),
        }
    }
    pub fn log_to_file_default(self, app: &str) -> LoggerBuilder3 {
        let fs_path = PathBuf::from_str("/var/local/log").unwrap().join(app);
        let fs = FileSpec::default()
            .directory(fs_path)
            .basename(app)
            .suffix("log");
        // 若为true，则会覆盖rotate中的数字、keep^
        self.log_to_file(
            fs,
            Criterion::AgeOrSize(Age::Day, 10_000_000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(10),
            true,
        )
    }
    pub fn log_to_file(
        self,
        fs: FileSpec,
        criterion: Criterion,
        naming: Naming,
        cleanup: Cleanup,
        append: bool,
    ) -> LoggerBuilder3 {
        LoggerBuilder3 {
            logger: self
                .logger
                .log_to_file(fs)
                .o_append(append)
                .rotate(criterion, naming, cleanup),
        }
    }
}
/// 控制台输出日志
pub fn logger_debug_default() -> LoggerHandle {
    LoggerBuilder::default(LevelFilter::Debug)
        .build_default()
        .log_to_stdout()
        .start()
}

/// 根据feature来确定日志输出
///     dev：控制台输出
///     prod：在目录/var/local/log/{app}输出日志；
///         每天或大小达到10m更换日志文件；
///         维持10个日志文件；
///         生成/var/local/etc/{app}/logspecification.toml的动态配置文件
pub fn logger_debug_feature(app: &str) -> LoggerHandle {
    _logger_debug_feature(app)
}
#[cfg(not(feature = "prod"))]
fn _logger_debug_feature(_app: &str) -> LoggerHandle {
    logger_debug_default()
}
#[cfg(feature = "prod")]
fn _logger_debug_feature(app: &str) -> LoggerHandle {
    let path = PathBuf::from_str("/var/local/etc/")
        .unwrap()
        .join(app)
        .join("logspecification.toml");

    let fs_path = PathBuf::from_str("/var/local/log").unwrap().join(app);
    let fs = FileSpec::default()
        .directory(fs_path)
        .basename(app)
        .suffix("log")
        // 若为true，则会覆盖rotate中的数字、keep^
        .use_timestamp(false);

    LoggerBuilder::default(LevelFilter::Debug)
        .build_default()
        .log_to_file(
            fs,
            Criterion::AgeOrSize(Age::Day, 10_000_000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(10),
            true,
        )
        .start_with_specfile(path)
}
