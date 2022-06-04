// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::fmt::format;

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("[{}]: {}", get_log_level_labell(level), message);
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}

fn get_log_level_labell(level: LogLevel) -> String {
    match level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    }
    .to_string()
}
