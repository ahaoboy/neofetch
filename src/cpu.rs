//! CPU information collector
//!
//! Collects CPU information including model name, core count, and frequency
//! across different platforms.

use crate::error::{NeofetchError, Result};
use std::fmt::Display;

/// CPU information structure
#[derive(Debug, Clone)]
pub struct Cpu {
    /// CPU model name
    pub name: String,
    /// Number of CPU cores
    pub cores: u32,
    /// CPU speed in MHz
    pub speed: u32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = vec![self.name.clone()];

        if self.cores > 0 {
            parts.push(format!("({})", self.cores));
        }
        if self.speed > 0 {
            parts.push(format!("{:.2} GHz", self.speed as f64 / 1000.));
        }

        write!(f, "{}", parts.join(" "))
    }
}

/// Get CPU information on Windows
#[cfg(windows)]
pub async fn get_cpu() -> Result<Cpu> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_Processor")]
    struct Processor {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "NumberOfLogicalProcessors")]
        number_of_cores: u32,
        #[serde(rename = "CurrentClockSpeed")]
        current_clock_speed: u32,
    }

    // Use WMI to query processor information
    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;
    let results: Vec<Processor> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    results
        .first()
        .map(|proc| Cpu {
            name: proc.name.trim().to_string(),
            cores: proc.number_of_cores,
            speed: proc.current_clock_speed,
        })
        .ok_or_else(|| NeofetchError::data_unavailable("No CPU information found"))
}

/// Get CPU information on Linux and macOS
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub async fn get_cpu() -> Result<Cpu> {
    use crate::utils::{parse_proc_file, read_file_to_string};

    let content = read_file_to_string("/proc/cpuinfo").await?;
    let cpuinfo = parse_proc_file(&content);

    // Get CPU model name
    let name = cpuinfo
        .get("model name")
        .or_else(|| cpuinfo.get("Hardware"))
        .ok_or_else(|| NeofetchError::data_unavailable("CPU model name not found"))?
        .to_string();

    let mut cpu = Cpu {
        name,
        cores: 0,
        speed: 0,
    };

    // Parse CPU frequency (MHz)
    if let Some(mhz_str) = cpuinfo.get("cpu MHz")
        && let Ok(mhz) = mhz_str.parse::<f64>() {
            cpu.speed = mhz as u32;
        }

    // Parse core count
    if let Some(cores_str) = cpuinfo.get("cpu cores")
        && let Ok(cores) = cores_str.parse::<u32>() {
            cpu.cores = cores;
        }

    Ok(cpu)
}

/// Get CPU information on Android
#[cfg(target_os = "android")]
pub async fn get_cpu() -> Result<Cpu> {
    use crate::utils::read_file_to_string;

    // Get SoC model from Android properties
    let soc_model = crate::share::get_property("ro.soc.model")
        .ok_or_else(|| NeofetchError::data_unavailable("SoC model property not found"))?;

    let name = crate::share::detect_cpu(&soc_model)
        .ok_or_else(|| NeofetchError::data_unavailable("CPU model not recognized"))?;

    let mut cpu = Cpu {
        name,
        cores: 0,
        speed: 0,
    };

    // Get core count from /sys/devices/system/cpu/present
    if let Ok(content) = read_file_to_string("/sys/devices/system/cpu/present").await {
        if let Some((left, right)) = content.trim().split_once('-') {
            if let (Ok(start), Ok(end)) = (left.parse::<u32>(), right.parse::<u32>()) {
                cpu.cores = end - start + 1;
            }
        }
    }

    // Calculate average CPU frequency
    if cpu.cores > 0 {
        let mut total_freq = 0u64;
        let mut freq_count = 0u32;

        for i in 0..cpu.cores {
            let freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", i);
            if let Ok(freq_str) = read_file_to_string(&freq_path).await {
                if let Ok(freq) = freq_str.trim().parse::<u32>() {
                    total_freq += freq as u64;
                    freq_count += 1;
                }
            }
        }

        if freq_count > 0 {
            // Convert from kHz to MHz
            cpu.speed = (total_freq / freq_count as u64 / 1024) as u32;
        }
    }

    Ok(cpu)
}
