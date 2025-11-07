//! Disk information collector
//!
//! Collects disk usage information for mounted filesystems.

use crate::error::{NeofetchError, Result};
use human_bytes::human_bytes;
use std::fmt::Display;

/// Disk information structure
#[derive(Debug, Clone)]
pub struct Disk {
    /// Disk name or mount point
    pub name: String,
    /// Total disk space in bytes
    pub total: u64,
    /// Used disk space in bytes
    pub used: u64,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let used = human_bytes(self.used as f64);
        let total = human_bytes(self.total as f64);
        let percent = if self.total > 0 {
            (self.used as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };
        write!(f, "{} / {} ({:.0}%)", used, total, percent)
    }
}

/// Get disk information on Windows
#[cfg(windows)]
pub async fn get_disk() -> Result<Vec<Disk>> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_logicaldisk")]
    struct Logicaldisk {
        #[serde(rename = "DeviceID")]
        device_id: String,
        #[serde(rename = "FreeSpace")]
        free_space: Option<u64>,
        #[serde(rename = "Size")]
        size: Option<u64>,
    }

    // Query WMI for disk information
    let results: Vec<Logicaldisk> = wmi_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let mut disks = Vec::new();
    for disk in results {
        if let (Some(size), Some(free_space)) = (disk.size, disk.free_space)
            && size > 0
        {
            disks.push(Disk {
                name: disk.device_id.trim_end_matches(':').to_string(),
                total: size,
                used: size.saturating_sub(free_space),
            });
        }
    }

    disks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(disks)
}

/// Get filesystem information for a specific path (Unix)
#[cfg(unix)]
fn get_filesystem_info(path: &str) -> Result<Disk> {
    use std::ffi::CString;

    let path_cstr = CString::new(path)
        .map_err(|_| NeofetchError::parse_error("path", "invalid path string"))?;

    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::statvfs(path_cstr.as_ptr(), &mut stat) };

    if result != 0 {
        return Err(NeofetchError::system_call(format!(
            "statvfs failed for {}",
            path
        )));
    }

    let total = stat.f_blocks as u64 * stat.f_frsize as u64;
    let available = stat.f_bavail as u64 * stat.f_frsize as u64;

    Ok(Disk {
        name: path.to_string(),
        total,
        used: total.saturating_sub(available),
    })
}

/// Get disk information on Unix-like systems
#[cfg(unix)]
pub async fn get_disk() -> Result<Vec<Disk>> {
    // Get root filesystem info
    let root_disk = get_filesystem_info("/")?;
    Ok(vec![root_disk])
}
