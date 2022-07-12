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
    pub fn _start(self) -> Result<LoggerHandle> {
        Ok(self.logger.start()?)
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

pub struct LoggerFeatureBuilder {
    _app: String,
    _debug_level: LevelFilter,
    _prod_level: LevelFilter,
    fs: FileSpec,
    criterion: Criterion,
    naming: Naming,
    cleanup: Cleanup,
    append: bool,
    modules: Vec<(String, LevelFilter)>,
}
impl LoggerFeatureBuilder {
    pub fn default(app: &str, _debug_level: LevelFilter, prod_level: LevelFilter) -> Self {
        let fs_path = PathBuf::from_str("/var/local/log").unwrap().join(app);
        let fs = FileSpec::default()
            .directory(fs_path)
            .basename(app)
            .suffix("log");
        // 若为true，则会覆盖rotate中的数字、keep^
        let criterion = Criterion::AgeOrSize(Age::Day, 10_000_000);
        let naming = Naming::Numbers;
        let cleanup = Cleanup::KeepLogFiles(10);
        let append = true;
        Self {
            _app: app.to_string(),
            _debug_level,
            _prod_level: prod_level,
            fs,
            criterion,
            naming,
            cleanup,
            append,
            modules: Vec::new(),
        }
    }
    pub fn module<M: AsRef<str>>(mut self, module_name: M, lf: LevelFilter) -> Self {
        self.modules.push((module_name.as_ref().to_owned(), lf));
        self
    }
    pub fn config(
        mut self,
        fs: FileSpec,
        criterion: Criterion,
        naming: Naming,
        cleanup: Cleanup,
        append: bool,
    ) -> Self {
        self.fs = fs;
        self.criterion = criterion;
        self.naming = naming;
        self.cleanup = cleanup;
        self.append = append;
        self
    }
    #[cfg(feature = "prod")]
    pub fn build(self) -> LoggerHandle {
        let mut log_spec_builder = LogSpecBuilder::new();
        log_spec_builder.default(self._prod_level);

        LoggerBuilder2 {
            logger: Logger::with(log_spec_builder.build())
                .format(with_thread)
                .write_mode(WriteMode::Direct),
        }
        .log_to_file(
            self.fs,
            self.criterion,
            self.naming,
            self.cleanup,
            self.append,
        )
        .start_with_specfile_default(self._app.as_str())
    }
    #[cfg(not(feature = "prod"))]
    pub fn build(self) -> LoggerHandle {
        let mut log_spec_builder = LogSpecBuilder::new();
        log_spec_builder.default(self._debug_level);
        LoggerBuilder2 {
            logger: Logger::with(log_spec_builder.build())
                .format(with_thread)
                .write_mode(WriteMode::Direct),
        }
        .log_to_stdout()
        .start()
    }
}
