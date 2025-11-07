//! File operation utilities
//!
//! Provides reusable functions for reading files with proper error handling
//! and parsing common file formats like /proc files.

use crate::error::{NeofetchError, Result};
use std::collections::HashMap;
use std::path::Path;

/// Read a file to string asynchronously with error context
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Result<String>` - File contents or error with context
///
/// # Example
/// ```no_run
/// use neofetch::utils::read_file_to_string;
///
/// # async fn example() -> neofetch::Result<()> {
/// let content = read_file_to_string("/proc/cpuinfo").await?;
/// # Ok(())
/// # }
/// ```
pub async fn read_file_to_string(path: impl AsRef<Path>) -> Result<String> {
    let path_ref = path.as_ref();
    tokio::fs::read_to_string(path_ref)
        .await
        .map_err(|e| NeofetchError::file_read(path_ref.display().to_string(), e))
}

/// Read a file to string synchronously with error context
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Result<String>` - File contents or error with context
///
/// # Example
/// ```no_run
/// use neofetch::utils::read_file_to_string_sync;
///
/// # fn example() -> neofetch::Result<()> {
/// let content = read_file_to_string_sync("/proc/cpuinfo")?;
/// # Ok(())
/// # }
/// ```
pub fn read_file_to_string_sync(path: impl AsRef<Path>) -> Result<String> {
    let path_ref = path.as_ref();
    std::fs::read_to_string(path_ref)
        .map_err(|e| NeofetchError::file_read(path_ref.display().to_string(), e))
}

/// Check if a file exists
///
/// # Arguments
/// * `path` - Path to check
///
/// # Returns
/// * `bool` - true if file exists, false otherwise
pub fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

/// Parse /proc-style files with key-value pairs separated by colons
///
/// # Arguments
/// * `content` - File content to parse
///
/// # Returns
/// * `HashMap<String, String>` - Parsed key-value pairs
///
/// # Example
/// ```
/// use neofetch::utils::parse_proc_file;
///
/// let content = "MemTotal:       16384 kB\nMemFree:        8192 kB";
/// let parsed = parse_proc_file(content);
/// assert_eq!(parsed.get("MemTotal"), Some(&"16384 kB".to_string()));
/// ```
pub fn parse_proc_file(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() || value.is_empty() {
                None
            } else {
                Some((key.to_string(), value.to_string()))
            }
        })
        .collect()
}

/// Parse key-value pairs with custom separator
///
/// # Arguments
/// * `content` - Content to parse
/// * `separator` - Separator character (e.g., '=', ':')
///
/// # Returns
/// * `HashMap<String, String>` - Parsed key-value pairs
pub fn parse_key_value(content: &str, separator: char) -> HashMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, separator);
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() {
                None
            } else {
                Some((key.to_string(), value.to_string()))
            }
        })
        .collect()
}

/// Extract filename from path
///
/// # Arguments
/// * `path` - File path
///
/// # Returns
/// * `Option<String>` - Filename without extension
pub fn get_filename(path: &str) -> Option<String> {
    let normalized = path.replace('\\', "/");
    let name = normalized.split('/').next_back()?;
    let name_without_ext = name.split('.').next()?;
    Some(name_without_ext.trim().to_string())
}

/// Read directory entries count
///
/// # Arguments
/// * `path` - Directory path
///
/// # Returns
/// * `Result<usize>` - Number of entries in directory
pub fn count_dir_entries(path: impl AsRef<Path>) -> Result<usize> {
    let path_ref = path.as_ref();
    std::fs::read_dir(path_ref)
        .map_err(|e| NeofetchError::file_read(path_ref.display().to_string(), e))
        .map(|entries| entries.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proc_file() {
        let content = "MemTotal:       16384 kB\nMemFree:        8192 kB\nBuffers:        1024 kB";
        let parsed = parse_proc_file(content);

        assert_eq!(parsed.get("MemTotal"), Some(&"16384 kB".to_string()));
        assert_eq!(parsed.get("MemFree"), Some(&"8192 kB".to_string()));
        assert_eq!(parsed.get("Buffers"), Some(&"1024 kB".to_string()));
    }

    #[test]
    fn test_parse_key_value() {
        let content = "NAME=\"Ubuntu\"\nVERSION=\"22.04\"";
        let parsed = parse_key_value(content, '=');

        assert_eq!(parsed.get("NAME"), Some(&"\"Ubuntu\"".to_string()));
        assert_eq!(parsed.get("VERSION"), Some(&"\"22.04\"".to_string()));
    }

    #[test]
    fn test_get_filename() {
        assert_eq!(get_filename("/usr/bin/bash"), Some("bash".to_string()));
        assert_eq!(
            get_filename("C:\\Windows\\System32\\cmd.exe"),
            Some("cmd".to_string())
        );
        assert_eq!(get_filename("file.txt"), Some("file".to_string()));
    }

    #[test]
    fn test_file_exists() {
        assert!(file_exists("/"));
        assert!(!file_exists("/this/path/should/not/exist"));
    }
}
