use chrono::Local;
use std::sync::Once;
use colored::*;

static INIT: Once = Once::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger;

impl Logger {
    pub fn init() {
        INIT.call_once(|| {
            // Initialize logging system
        });
    }

    pub fn log(level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let level_str = match level {
            LogLevel::Debug => "DEBUG".blue(),
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERROR".red(),
        };

        println!("[{}] {} - {}", timestamp, level_str, message);
    }

    pub fn debug(message: &str) {
        Self::log(LogLevel::Debug, message);
    }

    pub fn info(message: &str) {
        Self::log(LogLevel::Info, message);
    }

    pub fn warning(message: &str) {
        Self::log(LogLevel::Warning, message);
    }

    pub fn error(message: &str) {
        Self::log(LogLevel::Error, message);
    }
} 