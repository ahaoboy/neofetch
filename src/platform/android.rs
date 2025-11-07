//! Android-specific platform implementations
//!
//! Provides Android-specific helper functions for accessing system properties
//! and hardware information.

use crate::error::{NeofetchError, Result};
use std::ffi::{CStr, CString};

#[cfg(target_os = "android")]
unsafe extern "C" {
    /// FFI binding to Android's __system_property_get
    fn __system_property_get(name: *const libc::c_char, value: *mut libc::c_char) -> i32;
}

/// Get Android system property
///
/// # Arguments
/// * `property` - Property name (e.g., "ro.build.version.release")
///
/// # Returns
/// * `Result<String>` - Property value
///
/// # Example
/// ```no_run
/// use neofetch::platform::get_property;
///
/// # fn example() -> neofetch::Result<()> {
/// let android_version = get_property("ro.build.version.release")?;
/// # Ok(())
/// # }
/// ```
#[cfg(target_os = "android")]
pub fn get_property(property: &str) -> Result<String> {
    let prop_cstr = CString::new(property)
        .map_err(|_| NeofetchError::parse_error("property_name", "invalid string"))?;

    let mut buffer = [0i8; 92]; // Android property max length

    let result = unsafe {
        __system_property_get(
            prop_cstr.as_ptr() as *const u8,
            buffer.as_mut_ptr() as *mut u8,
        )
    };

    if result < 0 {
        return Err(NeofetchError::system_call(format!(
            "Failed to get property '{}'",
            property
        )));
    }

    let value = unsafe {
        CStr::from_ptr(buffer.as_ptr() as *const u8)
            .to_string_lossy()
            .into_owned()
    };

    if value.is_empty() {
        return Err(NeofetchError::data_unavailable(format!(
            "Property '{}' is empty",
            property
        )));
    }

    Ok(value)
}

/// Get Android version
#[cfg(target_os = "android")]
pub fn get_android_version() -> Result<String> {
    get_property("ro.build.version.release")
}

/// Get Android SDK version
#[cfg(target_os = "android")]
pub fn get_sdk_version() -> Result<String> {
    get_property("ro.build.version.sdk")
}

/// Get device manufacturer
#[cfg(target_os = "android")]
pub fn get_manufacturer() -> Result<String> {
    get_property("ro.product.manufacturer")
}

/// Get device model
#[cfg(target_os = "android")]
pub fn get_device_model() -> Result<String> {
    get_property("ro.product.model")
}

/// Get device brand
#[cfg(target_os = "android")]
pub fn get_device_brand() -> Result<String> {
    get_property("ro.product.brand")
}

/// Get SoC (System on Chip) model
#[cfg(target_os = "android")]
pub fn get_soc_model() -> Result<String> {
    get_property("ro.soc.model")
        .or_else(|_| get_property("ro.hardware"))
        .or_else(|_| get_property("ro.board.platform"))
}

/// Get ROM/firmware display ID
#[cfg(target_os = "android")]
pub fn get_rom_display_id() -> Result<String> {
    get_property("ro.build.display.id")
}

/// Get baseband version
#[cfg(target_os = "android")]
pub fn get_baseband_version() -> Result<String> {
    get_property("ro.baseband").or_else(|_| get_property("gsm.version.baseband"))
}

/// Get kernel version from /proc/version
#[cfg(target_os = "android")]
pub async fn get_kernel_version() -> Result<String> {
    use crate::utils::read_file_to_string;

    let content = read_file_to_string("/proc/version").await?;

    // Extract version from "Linux version X.X.X ..."
    if let Some(version_start) = content.find("Linux version ") {
        let version_str = &content[version_start + 14..];
        if let Some(space_pos) = version_str.find(' ') {
            return Ok(version_str[..space_pos].to_string());
        }
    }

    Err(NeofetchError::parse_error(
        "kernel_version",
        "invalid format",
    ))
}

/// Read CPU frequency for a specific core
#[cfg(target_os = "android")]
pub async fn get_cpu_freq(core: u32) -> Result<u32> {
    use crate::utils::read_file_to_string;

    let freq_path = format!(
        "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
        core
    );
    let content = read_file_to_string(&freq_path).await?;

    content
        .trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| {
            NeofetchError::parse_error("cpu_frequency", e.to_string())
        })
}

/// Get average CPU frequency across all cores
#[cfg(target_os = "android")]
pub async fn get_avg_cpu_freq(core_count: u32) -> Result<u32> {
    let mut total_freq = 0u64;
    let mut valid_cores = 0u32;

    for core in 0..core_count {
        if let Ok(freq) = get_cpu_freq(core).await {
            total_freq += freq as u64;
            valid_cores += 1;
        }
    }

    if valid_cores == 0 {
        return Err(NeofetchError::data_unavailable(
            "No CPU frequency data available",
        ));
    }

    Ok((total_freq / valid_cores as u64) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "android")]
    fn test_get_android_version() {
        let result = get_android_version();
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(target_os = "android")]
    fn test_get_device_model() {
        let result = get_device_model();
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(target_os = "android")]
    async fn test_get_kernel_version() {
        let result = get_kernel_version().await;
        assert!(result.is_ok());
    }
}
