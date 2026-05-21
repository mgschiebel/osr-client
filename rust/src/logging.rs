use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log level for typed errors.
#[derive(Debug, Serialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// Machine-parsable log entry.
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Log an error as JSON to stdout.
pub fn log_error(module: &str, message: &str, details: Option<serde_json::Value>) {
    let entry = LogEntry {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        level: LogLevel::Error,
        module: module.to_string(),
        message: message.to_string(),
        details,
    };
    if let Ok(json) = serde_json::to_string(&entry) {
        println!("{}", json);
    }
}

/// Log an info message as JSON to stdout.
pub fn log_info(module: &str, message: &str) {
    let entry = LogEntry {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        level: LogLevel::Info,
        module: module.to_string(),
        message: message.to_string(),
        details: None,
    };
    if let Ok(json) = serde_json::to_string(&entry) {
        println!("{}", json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry {
            timestamp: 1234567890,
            level: LogLevel::Error,
            module: "test".to_string(),
            message: "Test error".to_string(),
            details: None,
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("Test error"));
        assert!(json.contains("Error"));
    }
}
