//! Kernel version information collector
//!
//! Collects kernel version information across different platforms.

use crate::error::{NeofetchError, Result};

/// Get kernel version on Windows
#[cfg(windows)]
pub async fn get_kernel() -> Result<String> {
    use serde::Deserialize;
    use winreg::enums::*;
    use winreg::RegKey;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "Version")]
        version: String,
    }

    // Query kernel version from WMI
    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;
    let results: Vec<OperatingSystem> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let version = results
        .first()
        .ok_or_else(|| NeofetchError::data_unavailable("No kernel version found"))?
        .version
        .clone();

    let mut parts = vec![version];

    // Try to get display version from registry
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        && let Ok(display_version) = hklm.get_value::<String, _>("DisplayVersion")
    {
        parts.push(format!("({})", display_version));
    }

    Ok(parts.join(" "))
}

/// Get kernel version on Unix-like systems
#[cfg(unix)]
pub async fn get_kernel() -> Result<String> {
    use std::ffi::CStr;

    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::uname(&mut uts) };

    if result != 0 {
        return Err(NeofetchError::system_call("uname failed"));
    }

    let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    Ok(release)
}
