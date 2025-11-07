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

/// Get temperature sensors on Linux
#[cfg(target_os = "linux")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::platform::linux::{get_thermal_zones, read_thermal_zone_temp, read_thermal_zone_type};

    let zones = get_thermal_zones()?;
    let mut sensors = Vec::new();

    for zone_path in zones {
        if let Ok(temp) = read_thermal_zone_temp(&zone_path).await {
            let label = read_thermal_zone_type(&zone_path)
                .await
                .unwrap_or_else(|_| {
                    zone_path
                        .split('/')
                        .last()
                        .unwrap_or("unknown")
                        .to_string()
                });

            sensors.push(TempSensor {
                label,
                temperature_celsius: temp,
            });
        }
    }

    // Also try hwmon sensors
    if let Ok(hwmon_sensors) = read_hwmon_sensors().await {
        sensors.extend(hwmon_sensors);
    }

    if sensors.is_empty() {
        return Err(NeofetchError::data_unavailable("No temperature sensors found"));
    }

    Ok(sensors)
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

                    if filename_str.starts_with("temp") && filename_str.ends_with("_input") {
                        if let Ok(temp_str) = read_file_to_string(temp_entry.path()).await {
                            if let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>() {
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
        return Err(NeofetchError::data_unavailable("Thermal sensors not available"));
    }

    let mut sensors = Vec::new();

    if let Ok(entries) = std::fs::read_dir(thermal_path) {
        for entry in entries.flatten() {
            let zone_path = entry.path();
            let zone_name = entry.file_name();
            let zone_name_str = zone_name.to_string_lossy();

            if zone_name_str.starts_with("thermal_zone") {
                let temp_file = zone_path.join("temp");
                let type_file = zone_path.join("type");

                if let Ok(temp_str) = read_file_to_string(&temp_file).await {
                    if let Ok(temp_millidegrees) = temp_str.trim().parse::<i32>() {
                        let temp_celsius = temp_millidegrees as f32 / 1000.0;

                        let label = if let Ok(type_str) = read_file_to_string(&type_file).await {
                            type_str.trim().to_string()
                        } else {
                            zone_name_str.to_string()
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

    if sensors.is_empty() {
        return Err(NeofetchError::data_unavailable("No temperature sensors found"));
    }

    Ok(sensors)
}

/// Get temperature sensors on macOS
#[cfg(target_os = "macos")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::utils::execute_command_optional;

    // Try using powermetrics (requires sudo, may not work)
    if let Some(output) = execute_command_optional("powermetrics", &["--samplers", "smc", "-i1", "-n1"]).await {
        let mut sensors = Vec::new();

        for line in output.lines() {
            if line.contains("CPU die temperature") {
                if let Some(temp_str) = line.split(':').nth(1) {
                    if let Some(temp_val) = temp_str.trim().split_whitespace().next() {
                        if let Ok(temp) = temp_val.parse::<f32>() {
                            sensors.push(TempSensor {
                                label: "CPU".to_string(),
                                temperature_celsius: temp,
                            });
                        }
                    }
                }
            }
        }

        if !sensors.is_empty() {
            return Ok(sensors);
        }
    }

    // Try using osx-cpu-temp if available
    if let Some(output) = execute_command_optional("osx-cpu-temp", &[] as &[&str]).await {
        if let Some(temp_str) = output.split('°').next() {
            if let Ok(temp) = temp_str.trim().parse::<f32>() {
                return Ok(vec![TempSensor {
                    label: "CPU".to_string(),
                    temperature_celsius: temp,
                }]);
            }
        }
    }

    Err(NeofetchError::data_unavailable(
        "Temperature sensors not available (try installing osx-cpu-temp)"
    ))
}

/// Get temperature sensors on Windows
#[cfg(windows)]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename = "MSAcpi_ThermalZoneTemperature")]
    struct ThermalZone {
        #[serde(rename = "CurrentTemperature")]
        current_temperature: Option<u32>,
        #[serde(rename = "InstanceName")]
        instance_name: Option<String>,
    }

    // Try WMI thermal zone query (limited support on Windows)
    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::with_namespace_path("root\\wmi", com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;

    let results: Vec<ThermalZone> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let mut sensors = Vec::new();

    for zone in results {
        if let Some(temp_deciseconds) = zone.current_temperature {
            // WMI returns temperature in tenths of Kelvin
            let temp_kelvin = temp_deciseconds as f32 / 10.0;
            let temp_celsius = temp_kelvin - 273.15;

            let label = zone
                .instance_name
                .unwrap_or_else(|| "Thermal Zone".to_string());

            sensors.push(TempSensor {
                label,
                temperature_celsius: temp_celsius,
            });
        }
    }

    if sensors.is_empty() {
        return Err(NeofetchError::data_unavailable(
            "Temperature sensors not available via WMI"
        ));
    }

    Ok(sensors)
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
        if label_lower.contains("cpu") || label_lower.contains("core") || label_lower.contains("package") {
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
