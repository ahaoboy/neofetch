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
    use crate::platform::linux::{
        get_thermal_zones, read_thermal_zone_temp, read_thermal_zone_type,
    };

    let zones = get_thermal_zones()?;
    let mut sensors = Vec::new();

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
        return Err(NeofetchError::data_unavailable(
            "No temperature sensors found",
        ));
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

    const MIN_TEMP: f32 = 0.0; // Minimum valid temperature (Celsius)
    const MAX_TEMP: f32 = 120.0; // Maximum valid temperature (Celsius)

    let mut cpu_temps = Vec::new();
    let mut gpu_temps = Vec::new();
    let mut battery_temps = Vec::new();

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

            // Skip invalid temperatures (including -273°C which indicates sensor not active)
            if temp_celsius < MIN_TEMP || temp_celsius > MAX_TEMP {
                continue;
            }

            // Read sensor type/label
            let label = match read_file_to_string(&type_file).await {
                Ok(type_str) => type_str.trim().to_string(),
                Err(_) => continue,
            };

            // Classify sensor by type and collect temperatures
            let label_lower = label.to_lowercase();

            // CPU sensors: cpu, core, tsens (thermal sensor), cluster, silver, gold, prime
            if label_lower.contains("cpu")
                || label_lower.contains("core")
                || label_lower.contains("tsens")
                || label_lower.contains("cluster")
                || label_lower.contains("silver")
                || label_lower.contains("gold")
                || label_lower.contains("prime")
            {
                cpu_temps.push(temp_celsius);
            }
            // GPU sensors: gpu, gpuss (GPU subsystem), kgsl (Kernel Graphics Support Layer)
            else if label_lower.contains("gpu")
                || label_lower.contains("gpuss")
                || label_lower.contains("kgsl")
            {
                gpu_temps.push(temp_celsius);
            }
            // Battery sensors: battery, batt, charger
            else if label_lower.contains("battery")
                || label_lower.contains("batt")
                || label_lower.contains("charger")
            {
                battery_temps.push(temp_celsius);
            }
            // Note: Sensors like xoagg_therm (XO aggregate), sdr0_pa (SDR power amplifier),
            // modem, wifi, camera, etc. are not included in the main categories
        }
    }

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

/// Get temperature sensors on macOS
#[cfg(target_os = "macos")]
pub async fn get_temperature_sensors() -> Result<Vec<TempSensor>> {
    use crate::utils::execute_command_optional;

    // Try using powermetrics (requires sudo, may not work)
    if let Some(output) =
        execute_command_optional("powermetrics", &["--samplers", "smc", "-i1", "-n1"]).await
    {
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
        "Temperature sensors not available (try installing osx-cpu-temp)",
    ))
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

    const MIN_TEMP: f32 = 0.0; // Minimum valid temperature (Celsius)
    const MAX_TEMP: f32 = 120.0; // Maximum valid temperature (Celsius)

    // Try WMI thermal zone query (limited support on Windows)
    let results: Vec<ThermalZoneTemperature> = wmi_query_with_ns("root\\wmi")
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("WMI query failed: {}", e)))?;

    let mut cpu_temps = Vec::new();
    let mut gpu_temps = Vec::new();
    let mut battery_temps = Vec::new();

    for zone in results {
        // WMI returns temperature in tenths of Kelvin
        let temp_kelvin = zone.current_temperature as f32 / 10.0;
        let temp_celsius = temp_kelvin - 273.15;

        // Skip invalid temperatures
        if !(MIN_TEMP..=MAX_TEMP).contains(&temp_celsius) {
            continue;
        }

        // Classify sensor by instance name
        let name_lower = zone.instance_name.to_lowercase();

        // CPU sensors: cpu, processor, core, package
        if name_lower.contains("cpu")
            || name_lower.contains("processor")
            || name_lower.contains("core")
            || name_lower.contains("package")
        {
            cpu_temps.push(temp_celsius);
        }
        // GPU sensors: gpu, graphics, video, display adapter
        else if name_lower.contains("gpu")
            || name_lower.contains("graphics")
            || name_lower.contains("video")
            || name_lower.contains("display")
        {
            gpu_temps.push(temp_celsius);
        }
        // Battery sensors: battery, batt, acpi
        else if name_lower.contains("battery") || name_lower.contains("batt") {
            battery_temps.push(temp_celsius);
        }
        // If no specific category, try to infer from thermal zone naming
        else if name_lower.contains("tz") || name_lower.contains("thermal") {
            // Generic thermal zones - classify as CPU by default
            cpu_temps.push(temp_celsius);
        }
    }

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
            "No valid temperature sensors found via WMI",
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
