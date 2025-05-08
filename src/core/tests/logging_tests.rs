#[cfg(test)]
mod tests {
    use crate::core::logging::{Logger, LogLevel};
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            Logger::init();
        });
    }

    #[test]
    fn test_logger_init() {
        setup();
        // Just verify it doesn't panic
        Logger::init();
    }

    #[test]
    fn test_log_levels() {
        setup();
        // Test all log levels - these should not panic
        Logger::debug("Debug message");
        Logger::info("Info message");
        Logger::warning("Warning message");
        Logger::error("Error message");
    }

    #[test]
    fn test_direct_log() {
        setup();
        // Test direct log method with different levels
        Logger::log(LogLevel::Debug, "Direct debug message");
        Logger::log(LogLevel::Info, "Direct info message");
        Logger::log(LogLevel::Warning, "Direct warning message");
        Logger::log(LogLevel::Error, "Direct error message");
    }
} 