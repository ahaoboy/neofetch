//! System statistics collector
//!
//! Collects various system statistics like process count, load average, and boot time.

use crate::error::{NeofetchError, Result};
use std::fmt::Display;

/// System load average
#[derive(Debug, Clone, Copy)]
pub struct LoadAverage {
    /// 1-minute load average
    pub one_min: f32,
    /// 5-minute load average
    pub five_min: f32,
    /// 15-minute load average
    pub fifteen_min: f32,
}

impl Display for LoadAverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.2}, {:.2}, {:.2}",
            self.one_min, self.five_min, self.fifteen_min
        )
    }
}

/// Get process count on Unix-like systems
#[cfg(unix)]
pub async fn get_process_count() -> Result<usize> {
    let proc_path = std::path::Path::new("/proc");

    if !proc_path.exists() {
        return Err(NeofetchError::data_unavailable("/proc not available"));
    }

    let entries = std::fs::read_dir(proc_path)
        .map_err(|e| NeofetchError::file_read("/proc".to_string(), e))?;

    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .chars()
                .all(|c| c.is_ascii_digit())
        })
        .count();

    Ok(count)
}

/// Get process count on Windows
#[cfg(windows)]
pub async fn get_process_count() -> Result<usize> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_Process")]
    struct Process {
        #[serde(rename = "ProcessId")]
        #[allow(dead_code)]
        process_id: u32,
    }

    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;
    let results: Vec<Process> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    Ok(results.len())
}

/// Get load average on Unix-like systems
#[cfg(unix)]
pub async fn get_load_average() -> Result<LoadAverage> {
    use crate::utils::read_file_to_string;

    let content = read_file_to_string("/proc/loadavg").await?;
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(NeofetchError::parse_error(
            "loadavg",
            "insufficient data",
        ));
    }

    let one_min = parts[0]
        .parse::<f32>()
        .map_err(|e: std::num::ParseFloatError| {
            NeofetchError::parse_error("load_1min", e.to_string())
        })?;
    let five_min = parts[1]
        .parse::<f32>()
        .map_err(|e: std::num::ParseFloatError| {
            NeofetchError::parse_error("load_5min", e.to_string())
        })?;
    let fifteen_min = parts[2]
        .parse::<f32>()
        .map_err(|e: std::num::ParseFloatError| {
            NeofetchError::parse_error("load_15min", e.to_string())
        })?;

    Ok(LoadAverage {
        one_min,
        five_min,
        fifteen_min,
    })
}

/// Get load average on Windows (not supported)
#[cfg(windows)]
pub async fn get_load_average() -> Result<LoadAverage> {
    Err(NeofetchError::UnsupportedPlatform)
}

/// Get system boot time as Unix timestamp
#[cfg(unix)]
pub async fn get_boot_time() -> Result<i64> {
    use crate::utils::{parse_proc_file, read_file_to_string};

    let content = read_file_to_string("/proc/stat").await?;
    let stat_info = parse_proc_file(&content);

    let btime_str = stat_info
        .get("btime")
        .ok_or_else(|| NeofetchError::data_unavailable("btime not found in /proc/stat"))?;

    btime_str
        .split_whitespace()
        .next()
        .ok_or_else(|| NeofetchError::parse_error("btime", "missing value"))?
        .parse::<i64>()
        .map_err(|e: std::num::ParseIntError| {
            NeofetchError::parse_error("btime", e.to_string())
        })
}

/// Get system boot time on Windows
#[cfg(windows)]
pub async fn get_boot_time() -> Result<i64> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "LastBootUpTime")]
        last_boot_up_time: Option<String>,
    }

    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;
    let results: Vec<OperatingSystem> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let boot_time_str = results
        .first()
        .and_then(|os| os.last_boot_up_time.as_ref())
        .ok_or_else(|| NeofetchError::data_unavailable("Boot time not found"))?;

    // Parse WMI datetime format: YYYYMMDDHHMMSS.mmmmmm+UUU
    parse_wmi_datetime(boot_time_str)
}

/// Parse WMI datetime format to Unix timestamp
#[cfg(windows)]
fn parse_wmi_datetime(datetime_str: &str) -> Result<i64> {
    use chrono::{NaiveDateTime, TimeZone, Utc};

    // Extract the datetime part before the dot
    let datetime_part = datetime_str
        .split('.')
        .next()
        .ok_or_else(|| NeofetchError::parse_error("wmi_datetime", "invalid format"))?;

    if datetime_part.len() < 14 {
        return Err(NeofetchError::parse_error(
            "wmi_datetime",
            "insufficient length",
        ));
    }

    let year = &datetime_part[0..4];
    let month = &datetime_part[4..6];
    let day = &datetime_part[6..8];
    let hour = &datetime_part[8..10];
    let minute = &datetime_part[10..12];
    let second = &datetime_part[12..14];

    let datetime_string = format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second);

    let naive_dt = NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| NeofetchError::parse_error("wmi_datetime", e.to_string()))?;

    let dt = Utc.from_utc_datetime(&naive_dt);
    Ok(dt.timestamp())
}

/// Get formatted boot time string
pub async fn get_boot_time_formatted() -> Result<String> {
    let boot_timestamp = get_boot_time().await?;

    #[cfg(unix)]
    {
        use chrono::{Local, TimeZone};
        let dt = Local
            .timestamp_opt(boot_timestamp, 0)
            .single()
            .ok_or_else(|| NeofetchError::parse_error("boot_time", "invalid timestamp"))?;
        Ok(dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    #[cfg(windows)]
    {
        use chrono::{Local, TimeZone};
        let dt = Local
            .timestamp_opt(boot_timestamp, 0)
            .single()
            .ok_or_else(|| NeofetchError::parse_error("boot_time", "invalid timestamp"))?;
        Ok(dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

/// Get thread count (Unix only)
#[cfg(unix)]
pub async fn get_thread_count() -> Result<usize> {
    let proc_path = std::path::Path::new("/proc");

    if !proc_path.exists() {
        return Err(NeofetchError::data_unavailable("/proc not available"));
    }

    let mut total_threads = 0;

    if let Ok(entries) = std::fs::read_dir(proc_path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            // Check if it's a process directory (numeric)
            if name_str.chars().all(|c| c.is_ascii_digit()) {
                let task_dir = entry.path().join("task");
                if let Ok(task_entries) = std::fs::read_dir(task_dir) {
                    total_threads += task_entries.count();
                }
            }
        }
    }

    if total_threads == 0 {
        return Err(NeofetchError::data_unavailable("No threads found"));
    }

    Ok(total_threads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_process_count() {
        let result = get_process_count().await;
        assert!(result.is_ok());
        if let Ok(count) = result {
            assert!(count > 0);
        }
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn test_get_load_average() {
        let result = get_load_average().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_boot_time() {
        let result = get_boot_time().await;
        assert!(result.is_ok());
        if let Ok(timestamp) = result {
            assert!(timestamp > 0);
        }
    }
}
