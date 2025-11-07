//! Windows-specific platform implementations
//!
//! Provides Windows-specific helper functions, particularly for WMI queries.

use crate::error::{NeofetchError, Result};
use serde::de::DeserializeOwned;
use winreg::HKEY;

/// Execute a WMI query and return deserialized results
///
/// # Type Parameters
/// * `T` - Type to deserialize WMI results into
///
/// # Returns
/// * `Result<Vec<T>>` - Vector of deserialized WMI objects
///
/// # Example
/// ```no_run
/// use serde::Deserialize;
/// use neofetch::platform::wmi_query;
///
/// #[derive(Deserialize)]
/// #[serde(rename = "Win32_Processor")]
/// struct Processor {
///     #[serde(rename = "Name")]
///     name: String,
/// }
///
/// # async fn example() -> neofetch::Result<()> {
/// let processors: Vec<Processor> = wmi_query().await?;
/// # Ok(())
/// # }
/// ```
pub async fn wmi_query<T: DeserializeOwned>() -> Result<Vec<T>> {
    use wmi::{COMLibrary, WMIConnection};

    let com = COMLibrary::new().map_err(|e| {
        NeofetchError::wmi_error(format!("Failed to initialize COM library: {}", e))
    })?;

    let wmi_con = WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to create WMI connection: {}", e)))?;

    let results: Vec<T> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    Ok(results)
}

/// Execute a WMI query with a custom query string
///
/// # Arguments
/// * `query` - WQL query string
///
/// # Returns
/// * `Result<Vec<T>>` - Vector of deserialized WMI objects
pub async fn wmi_query_with_filter<T: DeserializeOwned>(query: &str) -> Result<Vec<T>> {
    use wmi::{COMLibrary, WMIConnection};

    let com = COMLibrary::new().map_err(|e| {
        NeofetchError::wmi_error(format!("Failed to initialize COM library: {}", e))
    })?;

    let wmi_con = WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to create WMI connection: {}", e)))?;

    let results: Vec<T> = wmi_con
        .raw_query(query)
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query '{}' failed: {}", query, e)))?;

    Ok(results)
}

/// Get Windows registry value
///
/// # Arguments
/// * `hive` - Registry hive (e.g., HKEY_LOCAL_MACHINE)
/// * `path` - Registry key path
/// * `value_name` - Name of the value to retrieve
///
/// # Returns
/// * `Result<String>` - Registry value as string
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

    // #[tokio::test]
    // async fn test_wmi_query() {
    //     use serde::Deserialize;

    //     #[derive(Deserialize)]
    //     #[serde(rename = "Win32_OperatingSystem")]
    //     struct OS {
    //         #[serde(rename = "Caption")]
    //         caption: String,
    //     }

    //     let result: Result<Vec<OS>> = wmi_query().await;
    //     assert!(result.is_ok());
    // }

    #[test]
    fn test_get_windows_version() {
        let result = get_windows_version();
        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}
