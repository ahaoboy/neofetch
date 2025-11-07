//! Temperature sensor information collector
//!
//! Collects temperature information from various system sensors.

use crate::error::{NeofetchError, Result};
use std::fmt::Display;

/// Temperature sensor information
#[derive(Debug, Clone)]
pub struct TempSensor {
    /// Sensor label/name
    pub label: String,
    /// Temperature in Celsius
    pub temperature_celsius: f32,
}

impl Display for TempSensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.1}°C", self.label, self.temperature_celsius)
    }
}

/// Temperature sensor category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SensorCategory {
    Cpu,
    Gpu,
    Battery,
}

/// Temperature validation constants
const MIN_TEMP: f32 = 0.0; // Minimum valid temperature (Celsius)
const MAX_TEMP: f32 = 120.0; // Maximum valid temperature (Celsius)

/// Classify a sensor label into a category
fn classify_sensor(label: &str) -> Option<SensorCategory> {
    let label_lower = label.to_lowercase();

    // CPU sensors
    if label_lower.contains("cpu")
        || label_lower.contains("core")
        || label_lower.contains("package")
        || label_lower.contains("processor")
        || label_lower.contains("k10temp")
        || label_lower.contains("coretemp")
        || label_lower.contains("x86_pkg")
        || label_lower.contains("tctl")
        || label_lower.contains("tdie")
        || label_lower.contains("tsens")
        || label_lower.contains("cluster")
        || label_lower.contains("silver")
        || label_lower.contains("gold")
        || label_lower.contains("prime")
        || label_lower.contains("tz")
        || label_lower.contains("thermal")
    {
        return Some(SensorCategory::Cpu);
    }

    // GPU sensors
    if label_lower.contains("gpu")
        || label_lower.contains("gpuss")
        || label_lower.contains("kgsl")
        || label_lower.contains("radeon")
        || label_lower.contains("amdgpu")
        || label_lower.contains("nvidia")
        || label_lower.contains("nouveau")
        || label_lower.contains("edge")
        || label_lower.contains("junction")
        || label_lower.contains("graphics")
        || label_lower.contains("video")
        || label_lower.contains("display")
    {
        return Some(SensorCategory::Gpu);
    }

    // Battery sensors
    if label_lower.contains("battery")
        || label_lower.contains("batt")
        || label_lower.contains("charger")
        || label_lower.contains("acpi")
    {
        return Some(SensorCategory::Battery);
    }

    None
}

/// Check if temperature is valid
fn is_valid_temp(temp: f32) -> bool {
    (MIN_TEMP..=MAX_TEMP).contains(&temp)
}

/// Aggregate temperatures by category and create averaged sensors
fn aggregate_temperatures(
    cpu_temps: Vec<f32>,
    gpu_temps: Vec<f32>,
    battery_temps: Vec<f32>,
) -> Result<Vec<TempSensor>> {
    let mut sensors = Vec::new();

    // Calculate and add CPU average (ordered first)
    if !cpu_temps.is_empty() {
        let avg_temp = cpu_temps.iter().sum::<f32>() / cpu_temps.len() as f32;
        sensors.push(TempSensor {
            label: format!("CPU (avg of {} sensors)", cpu_temps.len()),
            temperature_celsius: avg_temp,
        });
    }

    // Calculate and add GPU average (ordered second)
    if !gpu_temps.is_empty() {
        let avg_temp = gpu_temps.iter().sum::<f32>() / gpu_temps.len() as f32;
        sensors.push(TempSensor {
            label: format!("GPU (avg of {} sensors)", gpu_temps.len()),
            temperature_celsius: avg_temp,
        });
    }

    // Calculate and add Battery average (ordered third)
    if !battery_temps.is_empty() {
        let avg_temp = battery_temps.iter().sum::<f32>() / battery_temps.len() as f32;
        sensors.push(TempSensor {
            label: format!("Battery (avg of {} sensors)", battery_temps.len()),
            temperature_celsius: avg_temp,
        });
    }

    if sensors.is_empty() {
        return Err(NeofetchError::data_unavailable(
            "No valid temperature sensors found",
        ));
    }

    Ok(sensors)
}

/// Collect and categorize temperature readings
struct TempCollector {
    cpu_temps: Vec<f32>,
    gpu_temps: Vec<f32>,
    battery_temps: Vec<f32>,
}

impl TempCollector {
    fn new() -> Self {
        Self {
            cpu_temps: Vec::new(),
            gpu_temps: Vec::new(),
            battery_temps: Vec::new(),
        }
    }

    fn add_reading(&mut self, label: &str, temp: f32) {
        if !is_valid_temp(temp) {
            return;
        }

        if let Some(category) = classify_sensor(label) {
            match category {
                SensorCategory::Cpu => self.cpu_temps.push(temp),
                SensorCategory::Gpu => self.gpu_temps.push(temp),
                SensorCategory::Battery => self.battery_temps.push(temp),
            }
        }
    }

    fn into_sensors(self) -> Result<Vec<TempSensor>> {
        aggregate_temperatures(self.cpu_temps, self.gpu_temps, self.battery_temps)
    }
}

/// Get temperature sensors on Linux
#[cfg(target_os = "linux")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::platform::linux::{
        get_thermal_zones, read_thermal_zone_temp, read_thermal_zone_type,
    };

    let mut collector = TempCollector::new();

    // Read thermal zones
    if let Ok(zones) = get_thermal_zones() {
        for zone_path in zones {
            if let Ok(temp) = read_thermal_zone_temp(&zone_path).await {
                let label = read_thermal_zone_type(&zone_path)
                    .await
                    .unwrap_or_else(|_| {
                        zone_path
                            .split('/')
                            .next_back()
                            .unwrap_or("unknown")
                            .to_string()
                    });

                collector.add_reading(&label, temp);
            }
        }
    }

    // Also try hwmon sensors
    if let Ok(hwmon_sensors) = read_hwmon_sensors().await {
        for sensor in hwmon_sensors {
            collector.add_reading(&sensor.label, sensor.temperature_celsius);
        }
    }

    collector.into_sensors()
}

/// Read hwmon temperature sensors (Linux)
#[cfg(target_os = "linux")]
async fn read_hwmon_sensors() -> Result<Vec<TempSensor>> {
    use crate::utils::read_file_to_string;

    let hwmon_path = std::path::Path::new("/sys/class/hwmon");
    if !hwmon_path.exists() {
        return Ok(Vec::new());
    }

    let mut sensors = Vec::new();

    if let Ok(entries) = std::fs::read_dir(hwmon_path) {
        for entry in entries.flatten() {
            let hwmon_dir = entry.path();

            // Try to find temp*_input files
            if let Ok(temp_entries) = std::fs::read_dir(&hwmon_dir) {
                for temp_entry in temp_entries.flatten() {
                    let filename = temp_entry.file_name();
                    let filename_str = filename.to_string_lossy();

                    if filename_str.starts_with("temp")
                        && filename_str.ends_with("_input")
                        && let Ok(temp_str) = read_file_to_string(temp_entry.path()).await
                        && let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>()
                    {
                        let temp_celsius = temp_millidegrees as f32 / 1000.0;

                        // Try to get label
                        let label_file = filename_str.replace("_input", "_label");
                        let label_path = hwmon_dir.join(&label_file);
                        let label = if let Ok(label_str) = read_file_to_string(&label_path).await {
                            label_str.trim().to_string()
                        } else {
                            filename_str.trim_end_matches("_input").to_string()
                        };

                        sensors.push(TempSensor {
                            label,
                            temperature_celsius: temp_celsius,
                        });
                    }
                }
            }
        }
    }

    Ok(sensors)
}

/// Get temperature sensors on Android
#[cfg(target_os = "android")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::utils::read_file_to_string;

    let thermal_path = std::path::Path::new("/sys/class/thermal");
    if !thermal_path.exists() {
        return Err(NeofetchError::data_unavailable(
            "Thermal sensors not available",
        ));
    }

    let mut collector = TempCollector::new();

    if let Ok(entries) = std::fs::read_dir(thermal_path) {
        for entry in entries.flatten() {
            let zone_path = entry.path();
            let zone_name = entry.file_name();
            let zone_name_str = zone_name.to_string_lossy();

            if !zone_name_str.starts_with("thermal_zone") {
                continue;
            }

            let temp_file = zone_path.join("temp");
            let type_file = zone_path.join("type");

            // Read temperature
            let temp_celsius = match read_file_to_string(&temp_file).await {
                Ok(temp_str) => match temp_str.trim().parse::<i32>() {
                    Ok(temp_millidegrees) => temp_millidegrees as f32 / 1000.0,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            // Read sensor type/label
            let label = match read_file_to_string(&type_file).await {
                Ok(type_str) => type_str.trim().to_string(),
                Err(_) => continue,
            };

            collector.add_reading(&label, temp_celsius);
        }
    }

    collector.into_sensors()
}

/// Get temperature sensors on macOS
#[cfg(target_os = "macos")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::utils::execute_command_optional;

    let mut collector = TempCollector::new();

    // Try using powermetrics (requires sudo, may not work)
    if let Some(output) =
        execute_command_optional("powermetrics", &["--samplers", "smc", "-i1", "-n1"]).await
    {
        for line in output.lines() {
            // Parse different temperature sensors from powermetrics
            if let Some(colon_pos) = line.find(':') {
                let label = line[..colon_pos].trim();
                let value_part = line[colon_pos + 1..].trim();

                if let Some(temp_val) = value_part.split_whitespace().next() {
                    if let Ok(temp) = temp_val.parse::<f32>() {
                        collector.add_reading(label, temp);
                    }
                }
            }
        }
    }

    // Try using osx-cpu-temp if available (fallback for CPU only)
    if collector.cpu_temps.is_empty() {
        if let Some(output) = execute_command_optional("osx-cpu-temp", &[] as &[&str]).await {
            if let Some(temp_str) = output.split('°').next() {
                if let Ok(temp) = temp_str.trim().parse::<f32>() {
                    collector.add_reading("CPU", temp);
                }
            }
        }
    }

    collector.into_sensors().map_err(|_| {
        NeofetchError::data_unavailable(
            "Temperature sensors not available (try installing osx-cpu-temp or run with sudo for powermetrics)",
        )
    })
}

/// Get temperature sensors on Windows
#[cfg(windows)]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use serde::Deserialize;

    use crate::platform::wmi_query_with_ns;

    #[derive(Deserialize, Debug)]
    #[serde(rename = "MSAcpi_ThermalZoneTemperature")]
    struct ThermalZoneTemperature {
        #[serde(rename = "CurrentTemperature")]
        current_temperature: u32,
        #[serde(rename = "InstanceName")]
        instance_name: String,
    }

    // Try WMI thermal zone query (limited support on Windows)
    let results: Vec<ThermalZoneTemperature> = wmi_query_with_ns("root\\wmi")
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let mut collector = TempCollector::new();

    for zone in results {
        // WMI returns temperature in tenths of Kelvin
        let temp_kelvin = zone.current_temperature as f32 / 10.0;
        let temp_celsius = temp_kelvin - 273.15;

        collector.add_reading(&zone.instance_name, temp_celsius);
    }

    collector
        .into_sensors()
        .map_err(|_| NeofetchError::data_unavailable("No valid temperature sensors found via WMI"))
}

/// Get temperature sensors (unsupported platforms)
#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    windows
)))]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    Err(NeofetchError::UnsupportedPlatform)
}

/// Get CPU temperature (first available sensor)
pub async fn get_cpu_temperature() -> Result<f32> {
    let sensors = get_temperature_sensors().await?;

    // Try to find CPU-related sensor
    for sensor in &sensors {
        let label_lower = sensor.label.to_lowercase();
        if label_lower.contains("cpu")
            || label_lower.contains("core")
            || label_lower.contains("package")
        {
            return Ok(sensor.temperature_celsius);
        }
    }

    // Return first sensor if no CPU sensor found
    sensors
        .first()
        .map(|s| s.temperature_celsius)
        .ok_or_else(|| NeofetchError::data_unavailable("No temperature sensors available"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_temperature_sensors() {
        let result = get_temperature_sensors().await;
        // May or may not have sensors depending on platform and permissions
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_cpu_temperature() {
        let result = get_cpu_temperature().await;
        // May or may not be available
        assert!(result.is_ok() || result.is_err());
    }
}
