use log::{Level, LevelFilter, Log};

use crate::println;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let (color, level) = match record.level() {
            Level::Error => (31, "ERR"),
            Level::Warn  => (93, "WRN"),
            Level::Info  => (34, "INF"),
            Level::Debug => (32, "DBG"),
            Level::Trace => (90, "TRC"),
        };

        println!("\x1b[{}m{}\x1b[0m \x1b[1m{}\x1b[0m {}", color, level, record.target(), record.args());
    }

    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN")  => LevelFilter::Warn,
        Some("INFO")  => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}