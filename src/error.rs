//! Error types for the neofetch application
//!
//! This module provides a centralized error handling system using thiserror.
//! All errors in the application should use these types for consistent error handling.

use thiserror::Error;

/// Main error type for neofetch operations
#[derive(Error, Debug, Clone)]
pub enum NeofetchError {
    /// Error reading a file
    #[error("Failed to read file '{path}': {message}")]
    FileRead { path: String, message: String },

    /// Error writing to a file
    #[error("Failed to write file '{path}': {message}")]
    FileWrite { path: String, message: String },

    /// Error parsing data
    #[error("Failed to parse {field}: {reason}")]
    ParseError { field: String, reason: String },

    /// Error executing a command
    #[error("Failed to execute command '{command}': {message}")]
    CommandExecution { command: String, message: String },

    /// Command returned non-zero exit code
    #[error("Command '{command}' failed with exit code {code}")]
    CommandFailed { command: String, code: i32 },

    /// Error with WMI query on Windows
    #[cfg(windows)]
    #[error("WMI query failed: {0}")]
    WmiError(String),

    /// Error with system call
    #[error("System call failed: {0}")]
    SystemCall(String),

    /// Feature not available on current platform
    #[error("Feature not available on this platform")]
    UnsupportedPlatform,

    /// Data not available
    #[error("Data not available: {0}")]
    DataUnavailable(String),

    /// Invalid UTF-8 in string conversion
    #[error("Invalid UTF-8 data: {0}")]
    Utf8Error(String),

    /// Error from tokio task join
    #[error("Task join error: {0}")]
    JoinError(String),

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(String),

    /// Integer parsing error
    #[error("Failed to parse integer: {0}")]
    ParseInt(String),

    /// Float parsing error
    #[error("Failed to parse float: {0}")]
    ParseFloat(String),
}

// Implement From traits for common error types
impl From<std::io::Error> for NeofetchError {
    fn from(err: std::io::Error) -> Self {
        NeofetchError::Io(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for NeofetchError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        NeofetchError::Utf8Error(err.to_string())
    }
}

impl From<tokio::task::JoinError> for NeofetchError {
    fn from(err: tokio::task::JoinError) -> Self {
        NeofetchError::JoinError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for NeofetchError {
    fn from(err: std::num::ParseIntError) -> Self {
        NeofetchError::ParseInt(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for NeofetchError {
    fn from(err: std::num::ParseFloatError) -> Self {
        NeofetchError::ParseFloat(err.to_string())
    }
}

/// Result type alias for neofetch operations
pub type Result<T> = std::result::Result<T, NeofetchError>;

impl NeofetchError {
    /// Create a FileRead error with context
    pub fn file_read(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::FileRead {
            path: path.into(),
            message: source.to_string(),
        }
    }

    /// Create a FileWrite error with context
    pub fn file_write(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::FileWrite {
            path: path.into(),
            message: source.to_string(),
        }
    }

    /// Create a ParseError with context
    pub fn parse_error(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ParseError {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Create a CommandExecution error with context
    pub fn command_execution(command: impl Into<String>, source: std::io::Error) -> Self {
        Self::CommandExecution {
            command: command.into(),
            message: source.to_string(),
        }
    }

    /// Create a CommandFailed error with context
    pub fn command_failed(command: impl Into<String>, code: i32) -> Self {
        Self::CommandFailed {
            command: command.into(),
            code,
        }
    }

    /// Create a DataUnavailable error
    pub fn data_unavailable(message: impl Into<String>) -> Self {
        Self::DataUnavailable(message.into())
    }

    /// Create a SystemCall error
    pub fn system_call(message: impl Into<String>) -> Self {
        Self::SystemCall(message.into())
    }

    #[cfg(windows)]
    /// Create a WmiError
    pub fn wmi_error(message: impl Into<String>) -> Self {
        Self::WmiError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NeofetchError::parse_error("cpu_speed", "invalid number");
        assert_eq!(err.to_string(), "Failed to parse cpu_speed: invalid number");
    }

    #[test]
    fn test_data_unavailable() {
        let err = NeofetchError::data_unavailable("GPU information");
        assert_eq!(err.to_string(), "Data not available: GPU information");
    }
}
