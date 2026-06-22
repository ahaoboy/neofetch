use std::{ffi::OsStr, fmt::Debug};

use crate::error::{NeofetchError, Result};

pub fn exec<I, S>(cmd: S, args: I) -> Result<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    crate::utils::process::execute_command_sync(cmd, &args.into_iter().collect::<Vec<_>>())
}

pub async fn exec_async<S, I>(cmd: S, args: I) -> Result<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    crate::utils::process::execute_command(cmd, &args.into_iter().collect::<Vec<_>>()).await
}

pub fn get_file_name(path: &str) -> Result<String> {
    let path = path.replace('\\', "/");
    let name = path
        .split('/')
        .next_back()
        .and_then(|n| n.split('.').next())
        .map(|n| n.trim().to_string())
        .ok_or_else(|| {
            NeofetchError::parse_error(
                "path",
                format!("Failed to extract filename from '{}'", path),
            )
        })?;
    Ok(name)
}

pub fn get_pid_name(id: u32) -> Result<String> {
    let path = format!("/proc/{id}/comm");
    std::fs::read_to_string(&path)
        .map(|i| i.trim().to_string())
        .map_err(|e| NeofetchError::file_read(path, e))
}

pub fn get_ppid(id: u32) -> Result<u32> {
    use crate::utils::process::execute_command_sync;

    let status_path = format!("/proc/{id}/status");
    let ppid_line = execute_command_sync("grep", &["-i", "-F", "PPid:", status_path.as_str()])?;
    let ppid = ppid_line
        .split(':')
        .next_back()
        .ok_or_else(|| NeofetchError::parse_error("PPid", "missing colon separator"))?
        .trim();
    ppid.parse()
        .map_err(|e: std::num::ParseIntError| NeofetchError::parse_error("PPid", e.to_string()))
}

// Re-export Android property functions from platform module
#[cfg(target_os = "android")]
pub use crate::platform::get_property;

pub fn detect_cpu(name: &str) -> Result<String> {
    crate::mappings::CPU_MAPPINGS
        .iter()
        .find_map(|i| {
            if i.0 == name {
                Some(i.1.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            NeofetchError::data_unavailable(format!("CPU mapping not found for '{}'", name))
        })
}

pub fn detect_locale(name: &str) -> Result<String> {
    crate::mappings::LOCALE_MAPPINGS
        .iter()
        .find_map(|i| {
            if i.0 == name {
                Some(i.2.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            NeofetchError::data_unavailable(format!("Locale mapping not found for '{}'", name))
        })
}
