//! macOS-specific platform implementations
//!
//! Provides macOS-specific helper functions for system information gathering.

use crate::error::Result;
use crate::utils::execute_command;

/// Execute sysctl command to get system information
///
/// # Arguments
/// * `key` - sysctl key to query
///
/// # Returns
/// * `Result<String>` - sysctl value
///
/// # Example
/// ```no_run
/// use neofetch::platform::sysctl;
///
/// # async fn example() -> neofetch::Result<()> {
/// let cpu_brand = sysctl("machdep.cpu.brand_string").await?;
/// # Ok(())
/// # }
/// ```
pub async fn sysctl(key: &str) -> Result<String> {
    execute_command("sysctl", &["-n", key]).await
}

/// Get CPU brand string
pub async fn get_cpu_brand() -> Result<String> {
    sysctl("machdep.cpu.brand_string").await
}

/// Get CPU core count
pub async fn get_cpu_core_count() -> Result<u32> {
    let output = sysctl("hw.ncpu").await?;
    output.trim().parse().map_err(|e: std::num::ParseIntError| {
        crate::error::NeofetchError::parse_error("cpu_count", e.to_string())
    })
}

/// Get CPU frequency in MHz
pub async fn get_cpu_frequency() -> Result<u64> {
    let output = sysctl("hw.cpufrequency").await?;
    let hz: u64 = output
        .trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| {
            crate::error::NeofetchError::parse_error("cpu_frequency", e.to_string())
        })?;
    Ok(hz / 1_000_000) // Convert to MHz
}

/// Get total physical memory in bytes
pub async fn get_memory_total() -> Result<u64> {
    let output = sysctl("hw.memsize").await?;
    output.trim().parse().map_err(|e: std::num::ParseIntError| {
        crate::error::NeofetchError::parse_error("memory_size", e.to_string())
    })
}

/// Get macOS version using sw_vers
pub async fn get_macos_version() -> Result<String> {
    execute_command("sw_vers", &["-productVersion"]).await
}

/// Get macOS build version
pub async fn get_macos_build() -> Result<String> {
    execute_command("sw_vers", &["-buildVersion"]).await
}

/// Execute system_profiler command
///
/// # Arguments
/// * `data_type` - Type of data to profile (e.g., "SPHardwareDataType")
///
/// # Returns
/// * `Result<String>` - Profiler output
pub async fn system_profiler(data_type: &str) -> Result<String> {
    execute_command("system_profiler", &[data_type, "-json"]).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_sysctl() {
        let result = sysctl("kern.ostype").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "Darwin");
    }

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_get_cpu_brand() {
        let result = get_cpu_brand().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_get_macos_version() {
        let result = get_macos_version().await;
        assert!(result.is_ok());
    }
}
