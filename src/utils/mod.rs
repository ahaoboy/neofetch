//! Utility modules for common operations
//!
//! This module provides reusable utilities for file operations, process execution,
//! and platform detection to reduce code duplication across the codebase.

pub mod file;
pub mod platform;
pub mod process;

// Re-export commonly used functions
pub use file::{parse_proc_file, read_file_to_string, read_file_to_string_sync};
pub use platform::{Platform, current_platform, is_platform_available};
pub use process::{execute_command, execute_command_optional, execute_command_sync};
