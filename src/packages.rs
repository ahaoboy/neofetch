#[derive(Debug, Clone)]
pub struct Packages {
    snap: usize,
    dpkg: usize,
    pacman: usize,
    scoop: usize,
    opkg: usize,
}

impl Display for Packages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = Vec::new();
        if self.dpkg > 0 {
            v.push(format!("{} (dpkg)", self.dpkg))
        }
        if self.snap > 0 {
            v.push(format!("{} (snap)", self.snap))
        }
        if self.pacman > 0 {
            v.push(format!("{} (pacman)", self.pacman))
        }
        if self.scoop > 0 {
            v.push(format!("{} (scoop)", self.scoop))
        }
        if self.opkg > 0 {
            v.push(format!("{} (opkg)", self.opkg))
        }

        f.write_str(&v.join(", "))
    }
}

use std::fmt::Display;

use crate::error::{NeofetchError, Result};
use crate::utils::process::execute_command_sync;

/// Build a platform-aware Unix path (handles MSYS2 on Windows)
#[allow(unused_variables)]
fn unix_path(unix: &str, windows_msys: &str) -> std::path::PathBuf {
    #[cfg(unix)]
    {
        std::path::Path::new(unix).to_path_buf()
    }
    #[cfg(windows)]
    {
        let prefix = std::env::var("MSYSTEM_PREFIX").ok();
        let base = prefix.as_deref().unwrap_or("/");
        std::path::Path::new(base)
            .parent()
            .unwrap_or(std::path::Path::new("/"))
            .join(windows_msys)
    }
}

fn pacman() -> Result<usize> {
    let dir = unix_path("/var/lib/pacman/local", "var/lib/pacman/local");
    let count = std::fs::read_dir(&dir)
        .map_err(|e| NeofetchError::file_read(dir.display().to_string(), e))?
        .count();
    Ok(count.saturating_sub(1))
}

fn snap() -> Result<usize> {
    let dir = unix_path("/var/lib/snapd/snaps", "var/lib/snapd/snaps");
    let count = std::fs::read_dir(&dir)
        .map_err(|e| NeofetchError::file_read(dir.display().to_string(), e))?
        .count();
    Ok(count.saturating_sub(1))
}

fn scoop() -> Result<usize> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| NeofetchError::data_unavailable("Home directory not found"))?;
    let dir = home_dir.join("scoop").join("apps");
    let count = std::fs::read_dir(&dir)
        .map_err(|e| NeofetchError::file_read(dir.display().to_string(), e))?
        .count();
    Ok(count.saturating_sub(1))
}

fn dpkg() -> Result<usize> {
    let dir = unix_path("/var/lib/dpkg/status", "var/lib/dpkg/status");
    let file = std::fs::read_to_string(&dir)
        .map_err(|e| NeofetchError::file_read(dir.display().to_string(), e))?;
    let package_count = file
        .lines()
        .filter(|line| line.starts_with("Package:"))
        .count();
    Ok(package_count)
}

fn opkg() -> Result<usize> {
    let output = execute_command_sync("opkg", &["list-installed"])?;
    Ok(output.lines().count())
}

pub async fn get_packages() -> Result<Packages> {
    let packages = tokio::task::spawn_blocking(|| -> Result<Packages> {
        Ok(Packages {
            snap: snap().unwrap_or_default(),
            dpkg: dpkg().unwrap_or_default(),
            pacman: pacman().unwrap_or_default(),
            scoop: scoop().unwrap_or_default(),
            opkg: opkg().unwrap_or_default(),
        })
    })
    .await??;

    Ok(packages)
}
