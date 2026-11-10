#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogType {
    Verbose = 0,
    Debug = 1,
    Information = 2,
    Warning = 3,
    Error = 4,
}

pub trait LogSink: Send + Sync {
    fn log(&self, log_type: LogType, message: &str);
}

pub struct ConsoleLogSink;

impl LogSink for ConsoleLogSink {
    fn log(&self, log_type: LogType, message: &str) {
        let prefix = match log_type {
            LogType::Verbose => "VERBOSE",
            LogType::Debug => "DEBUG",
            LogType::Information => "INFO",
            LogType::Warning => "WARN",
            LogType::Error => "ERROR",
        };
        eprintln!("[{prefix}] {message}");
    }
}

use std::sync::{OnceLock, RwLock};

static LOG_SINKS: RwLock<Vec<Box<dyn LogSink>>> = RwLock::new(Vec::new());
static MINIMUM_LOG_TYPE: RwLock<LogType> = RwLock::new(LogType::Information);

pub fn write(log_type: LogType, message: &str) {
    let min = *MINIMUM_LOG_TYPE.read().unwrap();
    if log_type >= min {
        let sinks = LOG_SINKS.read().unwrap();
        for sink in sinks.iter() {
            sink.log(log_type, message);
        }
    }
}

pub fn add_log_sink(sink: Box<dyn LogSink>) {
    LOG_SINKS.write().unwrap().push(sink);
}

pub fn remove_all_log_sinks() {
    LOG_SINKS.write().unwrap().clear();
}

pub fn set_minimum_log_type(t: LogType) {
    *MINIMUM_LOG_TYPE.write().unwrap() = t;
}

pub fn init() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        add_log_sink(Box::new(ConsoleLogSink));
    });
}

#[macro_export]
macro_rules! log_verbose {
    ($($arg:tt)*) => {
        $crate::log::write($crate::log::LogType::Verbose, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::log::write($crate::log::LogType::Debug, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log::write($crate::log::LogType::Information, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::log::write($crate::log::LogType::Warning, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::log::write($crate::log::LogType::Error, &format!($($arg)*))
    };
}
