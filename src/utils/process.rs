//! Process execution utilities
//!
//! Provides reusable functions for executing commands with proper error handling
//! and output parsing.

use crate::error::{NeofetchError, Result};
use std::ffi::OsStr;
use std::process::Stdio;

/// Execute a command asynchronously and return its output
///
/// # Arguments
/// * `cmd` - Command to execute
/// * `args` - Command arguments
///
/// # Returns
/// * `Result<String>` - Command output (stdout) or error
///
/// # Example
/// ```no_run
/// use neofetch::utils::execute_command;
///
/// # async fn example() -> neofetch::Result<()> {
/// let output = execute_command("uname", &["-r"]).await?;
/// println!("Kernel: {}", output);
/// # Ok(())
/// # }
/// ```
pub async fn execute_command(cmd: impl AsRef<OsStr>, args: &[impl AsRef<OsStr>]) -> Result<String> {
    let cmd_ref = cmd.as_ref();
    let cmd_str = cmd_ref.to_string_lossy().to_string();

    let output = tokio::process::Command::new(cmd_ref)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .map_err(|e| NeofetchError::command_execution(cmd_str.clone(), e))?;

    if !output.status.success()
        && let Some(code) = output.status.code()
    {
        return Err(NeofetchError::command_failed(cmd_str, code));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Execute a command synchronously and return its output
///
/// # Arguments
/// * `cmd` - Command to execute
/// * `args` - Command arguments
///
/// # Returns
/// * `Result<String>` - Command output (stdout) or error
///
/// # Example
/// ```no_run
/// use neofetch::utils::execute_command_sync;
///
/// # fn example() -> neofetch::Result<()> {
/// let output = execute_command_sync("uname", &["-r"])?;
/// println!("Kernel: {}", output);
/// # Ok(())
/// # }
/// ```
pub fn execute_command_sync(cmd: impl AsRef<OsStr>, args: &[impl AsRef<OsStr>]) -> Result<String> {
    let cmd_ref = cmd.as_ref();
    let cmd_str = cmd_ref.to_string_lossy().to_string();

    let output = std::process::Command::new(cmd_ref)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| NeofetchError::command_execution(cmd_str.clone(), e))?;

    if !output.status.success()
        && let Some(code) = output.status.code()
    {
        return Err(NeofetchError::command_failed(cmd_str, code));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Execute a command and return output, ignoring errors
///
/// # Arguments
/// * `cmd` - Command to execute
/// * `args` - Command arguments
///
/// # Returns
/// * `Option<String>` - Command output or None if failed
pub async fn execute_command_optional(
    cmd: impl AsRef<OsStr>,
    args: &[impl AsRef<OsStr>],
) -> Option<String> {
    execute_command(cmd, args).await.ok()
}

/// Execute a command synchronously and return output, ignoring errors
///
/// # Arguments
/// * `cmd` - Command to execute
/// * `args` - Command arguments
///
/// # Returns
/// * `Option<String>` - Command output or None if failed
pub fn execute_command_sync_optional(
    cmd: impl AsRef<OsStr>,
    args: &[impl AsRef<OsStr>],
) -> Option<String> {
    execute_command_sync(cmd, args).ok()
}

/// Get process name from PID (Unix only)
///
/// # Arguments
/// * `pid` - Process ID
///
/// # Returns
/// * `Result<String>` - Process name
#[cfg(unix)]
pub fn get_process_name(pid: u32) -> Result<String> {
    use crate::utils::read_file_to_string_sync;
    let path = format!("/proc/{}/comm", pid);
    read_file_to_string_sync(&path).map(|s| s.trim().to_string())
}

/// Get parent process ID (Unix only)
///
/// # Arguments
/// * `pid` - Process ID
///
/// # Returns
/// * `Result<u32>` - Parent process ID
#[cfg(unix)]
pub fn get_parent_pid(pid: u32) -> Result<u32> {
    let output = execute_command_sync(
        "grep",
        &["-i", "-F", "PPid:", &format!("/proc/{}/status", pid)],
    )?;

    let ppid_str = output
        .split(':')
        .nth(1)
        .ok_or_else(|| NeofetchError::parse_error("PPid", "missing colon separator"))?
        .trim();

    ppid_str
        .parse()
        .map_err(|e| NeofetchError::parse_error("PPid", format!("invalid number: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_command() {
        // Test with a command that should exist on all platforms
        #[cfg(unix)]
        let result = execute_command("echo", &["test"]).await;
        #[cfg(windows)]
        let result = execute_command("cmd", &["/C", "echo test"]).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_command_sync() {
        #[cfg(unix)]
        let result = execute_command_sync("echo", &["test"]);
        #[cfg(windows)]
        let result = execute_command_sync("cmd", &["/C", "echo test"]);

        assert!(result.is_ok());
    }
}
