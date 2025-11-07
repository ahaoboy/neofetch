//! Platform detection and utilities
//!
//! Provides functions for detecting the current platform and checking
//! feature availability across different operating systems.

/// Supported platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Windows operating system
    Windows,
    /// Linux operating system
    Linux,
    /// macOS operating system
    MacOS,
    /// Android operating system
    Android,
    /// OpenWrt embedded Linux
    OpenWrt,
    /// FreeBSD operating system
    FreeBSD,
    /// NetBSD operating system
    NetBSD,
    /// OpenBSD operating system
    OpenBSD,
    /// Unknown or unsupported platform
    Unknown,
}

impl Platform {
    /// Get the current platform
    ///
    /// # Returns
    /// * `Platform` - The detected platform
    ///
    /// # Example
    /// ```
    /// use neofetch::utils::Platform;
    ///
    /// let platform = Platform::current();
    /// println!("Running on: {:?}", platform);
    /// ```
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "android")]
        return Platform::Android;

        #[cfg(target_os = "linux")]
        {
            // Check for OpenWrt
            if std::path::Path::new("/etc/openwrt_release").exists()
                || std::path::Path::new("/etc/openwrt_version").exists() {
                return Platform::OpenWrt;
            }
            return Platform::Linux;
        }

        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "freebsd")]
        return Platform::FreeBSD;

        #[cfg(target_os = "netbsd")]
        return Platform::NetBSD;

        #[cfg(target_os = "openbsd")]
        return Platform::OpenBSD;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        Platform::Unknown
    }

    /// Check if the platform is Unix-like
    pub fn is_unix(&self) -> bool {
        matches!(
            self,
            Platform::Linux
                | Platform::MacOS
                | Platform::Android
                | Platform::OpenWrt
                | Platform::FreeBSD
                | Platform::NetBSD
                | Platform::OpenBSD
        )
    }

    /// Check if the platform is Windows
    pub fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }

    /// Check if the platform is Linux-based
    pub fn is_linux_based(&self) -> bool {
        matches!(self, Platform::Linux | Platform::Android | Platform::OpenWrt)
    }

    /// Check if the platform is BSD-based
    pub fn is_bsd(&self) -> bool {
        matches!(
            self,
            Platform::FreeBSD | Platform::NetBSD | Platform::OpenBSD
        )
    }

    /// Get platform name as string
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::MacOS => "macOS",
            Platform::Android => "Android",
            Platform::OpenWrt => "OpenWrt",
            Platform::FreeBSD => "FreeBSD",
            Platform::NetBSD => "NetBSD",
            Platform::OpenBSD => "OpenBSD",
            Platform::Unknown => "Unknown",
        }
    }
}

/// Get the current platform
///
/// # Returns
/// * `Platform` - The detected platform
pub fn current_platform() -> Platform {
    Platform::current()
}

/// Check if a feature is available on the current platform
///
/// # Arguments
/// * `feature` - Feature name to check
///
/// # Returns
/// * `bool` - true if feature is available
pub fn is_platform_available(feature: &str) -> bool {
    let platform = Platform::current();

    match feature {
        "wmi" => platform.is_windows(),
        "proc" => platform.is_linux_based(),
        "sysctl" => platform.is_bsd() || platform == Platform::MacOS,
        "display_info" => !matches!(platform, Platform::Android | Platform::OpenWrt),
        "battery" => !matches!(platform, Platform::OpenWrt),
        "gpu" => !matches!(platform, Platform::OpenWrt),
        "temperature" => true, // Available on most platforms
        "network" => true,     // Available on all platforms
        _ => false,
    }
}

/// Macro to conditionally compile code based on platform
///
/// # Example
/// ```ignore
/// platform_specific! {
///     windows => {
///         // Windows-specific code
///     },
///     unix => {
///         // Unix-specific code
///     }
/// }
/// ```
#[macro_export]
macro_rules! platform_specific {
    (
        $(
            $platform:ident => $code:block
        ),* $(,)?
    ) => {
        $(
            #[cfg(target_os = stringify!($platform))]
            $code
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_platform() {
        let platform = Platform::current();
        assert_ne!(platform, Platform::Unknown);
    }

    #[test]
    fn test_platform_properties() {
        let platform = Platform::current();

        #[cfg(unix)]
        assert!(platform.is_unix());

        #[cfg(windows)]
        assert!(platform.is_windows());

        #[cfg(target_os = "linux")]
        assert!(platform.is_linux_based());
    }

    #[test]
    fn test_platform_name() {
        let platform = Platform::current();
        let name = platform.name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_feature_availability() {
        #[cfg(windows)]
        assert!(is_platform_available("wmi"));

        #[cfg(target_os = "linux")]
        assert!(is_platform_available("proc"));

        assert!(is_platform_available("network"));
    }
}
