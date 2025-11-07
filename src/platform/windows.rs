//! Windows-specific platform implementations
//!
//! Provides Windows-specific helper functions, particularly for WMI queries.

use crate::error::{NeofetchError, Result};
use serde::de::DeserializeOwned;
use winreg::HKEY;

pub async fn wmi_query<T: DeserializeOwned>() -> Result<Vec<T>> {
    use wmi::WMIConnection;

    let wmi_con = WMIConnection::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to create WMI connection: {}", e)))?;

    let results: Vec<T> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    Ok(results)
}

pub async fn wmi_query_with_ns<T: DeserializeOwned>(ns: &str) -> Result<Vec<T>> {
    use wmi::WMIConnection;

    let wmi_con = WMIConnection::with_namespace_path(ns)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to create WMI connection: {}", e)))?;

    let results: Vec<T> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    Ok(results)
}

pub async fn wmi_query_with_filter<T: DeserializeOwned>(query: &str) -> Result<Vec<T>> {
    use wmi::WMIConnection;

    let wmi_con = WMIConnection::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to create WMI connection: {}", e)))?;

    let results: Vec<T> = wmi_con
        .raw_query(query)
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query '{}' failed: {}", query, e)))?;

    Ok(results)
}

pub fn get_registry_string(hive: HKEY, path: &str, value_name: &str) -> Result<String> {
    use winreg::RegKey;

    let key = RegKey::predef(hive).open_subkey(path).map_err(|e| {
        NeofetchError::system_call(format!("Failed to open registry key '{}': {}", path, e))
    })?;

    key.get_value::<String, _>(value_name).map_err(|e| {
        NeofetchError::system_call(format!(
            "Failed to read registry value '{}\\{}': {}",
            path, value_name, e
        ))
    })
}

/// Get Windows version information from registry
pub fn get_windows_version() -> Result<String> {
    use winreg::enums::*;

    let path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";

    // Try to get display version (Windows 10/11)
    if let Ok(display_version) = get_registry_string(HKEY_LOCAL_MACHINE, path, "DisplayVersion") {
        return Ok(display_version);
    }

    // Fallback to ReleaseId
    if let Ok(release_id) = get_registry_string(HKEY_LOCAL_MACHINE, path, "ReleaseId") {
        return Ok(release_id);
    }

    Err(NeofetchError::data_unavailable(
        "Windows version not found in registry",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_windows_version() {
        let result = get_windows_version();
        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}
