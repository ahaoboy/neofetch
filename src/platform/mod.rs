//! Platform-specific implementations
//!
//! This module provides platform-specific abstractions and helper functions
//! for different operating systems.

#[cfg(windows)]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "android")]
pub mod android;

// Re-export platform-specific functions
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "android")]
pub use android::*;
