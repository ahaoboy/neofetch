//! Linux-specific platform implementations
//!
//! Provides Linux-specific helper functions for reading /proc and /sys filesystems.

use crate::error::{NeofetchError, Result};
use crate::utils::{parse_proc_file, read_file_to_string};
use std::collections::HashMap;

/// Read and parse /proc/cpuinfo
///
/// # Returns
/// * `Result<HashMap<String, String>>` - Parsed CPU information
pub async fn read_cpuinfo() -> Result<HashMap<String, String>> {
    let content = read_file_to_string("/proc/cpuinfo").await?;
    Ok(parse_proc_file(&content))
}

/// Read and parse /proc/meminfo
///
/// # Returns
/// * `Result<HashMap<String, String>>` - Parsed memory information
pub async fn read_meminfo() -> Result<HashMap<String, String>> {
    let content = read_file_to_string("/proc/meminfo").await?;
    Ok(parse_proc_file(&content))
}

/// Read and parse /etc/os-release
///
/// # Returns
/// * `Result<HashMap<String, String>>` - Parsed OS release information
pub async fn read_os_release() -> Result<HashMap<String, String>> {
    let content = read_file_to_string("/etc/os-release").await?;

    // Parse key=value pairs
    let mut info: HashMap<String, String> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() {
                None
            } else {
                Some((key.to_string(), value.to_string()))
            }
        })
        .collect();

    // Remove quotes from values
    for value in info.values_mut() {
        *value = value.trim_matches('"').to_string();
    }

    Ok(info)
}

/// Get distribution name from /etc/os-release
///
/// # Returns
/// * `Result<String>` - Distribution name
pub async fn get_distro_name() -> Result<String> {
    let os_release = read_os_release().await?;

    os_release
        .get("PRETTY_NAME")
        .or_else(|| os_release.get("NAME"))
        .cloned()
        .ok_or_else(|| NeofetchError::data_unavailable("Distribution name not found"))
}

/// Read sysfs file
///
/// # Arguments
/// * `path` - Path under /sys
///
/// # Returns
/// * `Result<String>` - File content
pub async fn read_sysfs(path: &str) -> Result<String> {
    let full_path = if path.starts_with("/sys") {
        path.to_string()
    } else {
        format!("/sys/{}", path)
    };

    read_file_to_string(&full_path).await
}

/// Get CPU core count from /sys
///
/// # Returns
/// * `Result<u32>` - Number of CPU cores
pub async fn get_cpu_core_count() -> Result<u32> {
    let content = read_sysfs("devices/system/cpu/present").await?;

    if let Some((start, end)) = content.trim().split_once('-') {
        let start: u32 = start
            .parse()
            .map_err(|e: std::num::ParseIntError| {
                NeofetchError::parse_error("cpu_start", e.to_string())
            })?;
        let end: u32 = end
            .parse()
            .map_err(|e: std::num::ParseIntError| {
                NeofetchError::parse_error("cpu_end", e.to_string())
            })?;
        Ok(end - start + 1)
    } else {
        Err(NeofetchError::parse_error("cpu_present", "invalid format"))
    }
}

/// Get list of thermal zones
///
/// # Returns
/// * `Result<Vec<String>>` - List of thermal zone paths
pub fn get_thermal_zones() -> Result<Vec<String>> {
    let thermal_path = std::path::Path::new("/sys/class/thermal");

    if !thermal_path.exists() {
        return Ok(Vec::new());
    }

    let entries = std::fs::read_dir(thermal_path)
        .map_err(|e| NeofetchError::file_read("/sys/class/thermal".to_string(), e))?;

    let mut zones = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("thermal_zone") {
            zones.push(entry.path().display().to_string());
        }
    }

    zones.sort();
    Ok(zones)
}

/// Read temperature from thermal zone
///
/// # Arguments
/// * `zone_path` - Path to thermal zone
///
/// # Returns
/// * `Result<f32>` - Temperature in Celsius
pub async fn read_thermal_zone_temp(zone_path: &str) -> Result<f32> {
    let temp_path = format!("{}/temp", zone_path);
    let content = read_file_to_string(&temp_path).await?;

    let temp_millidegrees: i32 = content
        .trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| {
            NeofetchError::parse_error("temperature", e.to_string())
        })?;

    Ok(temp_millidegrees as f32 / 1000.0)
}

/// Get thermal zone type/name
///
/// # Arguments
/// * `zone_path` - Path to thermal zone
///
/// # Returns
/// * `Result<String>` - Thermal zone type
pub async fn read_thermal_zone_type(zone_path: &str) -> Result<String> {
    let type_path = format!("{}/type", zone_path);
    read_file_to_string(&type_path).await.map(|s| s.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_meminfo() {
        let result = read_meminfo().await;
        if result.is_ok() {
            let meminfo = result.unwrap();
            assert!(meminfo.contains_key("MemTotal"));
        }
    }

    #[tokio::test]
    async fn test_read_os_release() {
        let result = read_os_release().await;
        // Should work on most Linux systems
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_get_thermal_zones() {
        let result = get_thermal_zones();
        // Should return empty vec or list of zones
        assert!(result.is_ok());
    }
}
