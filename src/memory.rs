//! Memory information collector
//!
//! Collects memory usage information including total, used, and available memory.

use crate::error::{NeofetchError, Result};

/// Get memory information on Linux / Android
#[cfg(any(target_os = "linux", target_os = "android"))]
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
    let usage_percent = (used_kb / total_kb * 100.0) as u32;

    use human_bytes::human_bytes;
    Ok(format!(
        "{} / {} ({}%)",
        human_bytes(used_kb * 1024.0),
        human_bytes(total_kb * 1024.0),
        usage_percent,
    ))
}

/// Get memory information on macOS
#[cfg(target_os = "macos")]
pub async fn get_memory() -> Result<String> {
    use crate::platform::macos;
    use crate::utils::execute_command;

    let total_bytes = macos::get_memory_total().await?;

    // Parse vm_stat for memory usage
    let vm_stat = execute_command("vm_stat", &[]).await?;
    let page_size_bytes: u64 = 16384; // Default macOS page size

    let parse_vm_stat = |key: &str| -> Option<u64> {
        vm_stat
            .lines()
            .find(|l| l.trim_start().starts_with(key))
            .and_then(|l| l.rsplit(':').next())
            .and_then(|v| v.trim().trim_end_matches('.').parse::<u64>().ok())
    };

    let free_pages = parse_vm_stat("Pages free").unwrap_or(0);
    let active_pages = parse_vm_stat("Pages active").unwrap_or(0);
    let inactive_pages = parse_vm_stat("Pages inactive").unwrap_or(0);
    let wired_pages = parse_vm_stat("Pages wired down").unwrap_or(0);
    let speculative_pages = parse_vm_stat("Pages speculative").unwrap_or(0);

    let used_pages = active_pages + wired_pages + speculative_pages;
    let free_total_pages = free_pages + inactive_pages;
    let used_bytes = used_pages * page_size_bytes;

    let usage_percent = (used_bytes as f64 / total_bytes as f64 * 100.0) as u32;

    use human_bytes::human_bytes;
    Ok(format!(
        "{} / {} ({}%)",
        human_bytes(used_bytes as f64),
        human_bytes(total_bytes as f64),
        usage_percent,
    ))
}

/// Get memory information on other Unix systems
#[cfg(all(unix, not(any(target_os = "linux", target_os = "android", target_os = "macos"))))]
pub async fn get_memory() -> Result<String> {
    Err(NeofetchError::UnsupportedPlatform)
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
    let results: Vec<OperatingSystem> = wmi_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let info = results
        .first()
        .ok_or_else(|| NeofetchError::data_unavailable("No memory information found"))?;

    let used_kb = (info.total_visible_memory_size - info.free_physical_memory) as f64;
    let total_kb = info.total_visible_memory_size as f64;
    let usage_percent = (used_kb / total_kb * 100.0) as u32;

    use human_bytes::human_bytes;

    use crate::platform::wmi_query;
    Ok(format!(
        "{} / {} ({}%)",
        human_bytes(used_kb * 1024.0),
        human_bytes(total_kb * 1024.0),
        usage_percent
    ))
}
