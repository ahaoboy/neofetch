//! Memory information collector
//!
//! Collects memory usage information including total, used, and available memory.

use crate::error::{NeofetchError, Result};

/// Get memory information on Unix-like systems
#[cfg(not(windows))]
pub async fn get_memory() -> Result<String> {
    use crate::utils::{parse_proc_file, read_file_to_string};

    let content = read_file_to_string("/proc/meminfo").await?;
    let meminfo = parse_proc_file(&content);

    // Parse total memory
    let total_str = meminfo
        .get("MemTotal")
        .ok_or_else(|| NeofetchError::data_unavailable("MemTotal not found"))?;
    let total_kb = total_str
        .split_whitespace()
        .next()
        .ok_or_else(|| NeofetchError::parse_error("MemTotal", "missing value"))?
        .parse::<f64>()
        .map_err(|e| NeofetchError::parse_error("MemTotal", e.to_string()))?;

    // Parse free memory
    let free_str = meminfo
        .get("MemFree")
        .ok_or_else(|| NeofetchError::data_unavailable("MemFree not found"))?;
    let free_kb = free_str
        .split_whitespace()
        .next()
        .ok_or_else(|| NeofetchError::parse_error("MemFree", "missing value"))?
        .parse::<f64>()
        .map_err(|e| NeofetchError::parse_error("MemFree", e.to_string()))?;

    // Calculate used memory
    let used_kb = total_kb - free_kb;

    use human_bytes::human_bytes;
    Ok(format!(
        "{} / {}",
        human_bytes(used_kb * 1024.0),
        human_bytes(total_kb * 1024.0),
    ))
}

/// Get memory information on Windows
#[cfg(windows)]
pub async fn get_memory() -> Result<String> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "TotalVisibleMemorySize")]
        total_visible_memory_size: u64,
        #[serde(rename = "FreePhysicalMemory")]
        free_physical_memory: u64,
    }

    // Query WMI for memory information
    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;
    let results: Vec<OperatingSystem> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let info = results
        .first()
        .ok_or_else(|| NeofetchError::data_unavailable("No memory information found"))?;

    let used_kb = (info.total_visible_memory_size - info.free_physical_memory) as f64;
    let total_kb = info.total_visible_memory_size as f64;
    let usage_percent = (used_kb / total_kb * 100.0) as u32;

    use human_bytes::human_bytes;
    Ok(format!(
        "{} / {} ({}%)",
        human_bytes(used_kb * 1024.0),
        human_bytes(total_kb * 1024.0),
        usage_percent
    ))
}
