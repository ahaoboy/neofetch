#[derive(Debug, Clone)]
pub struct Packages {
    snap: usize,
    dpkg: usize,
    pacman: usize,
    scoop: usize,
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

        f.write_str(&v.join(", "))
    }
}

use std::fmt::Display;

fn pacman() -> Option<usize> {
    #[cfg(unix)]
    let dir = std::path::Path::new("/var/lib/pacman/local");
    #[cfg(windows)]
    let dir = std::path::Path::new(&std::env::var("MSYSTEM_PREFIX").ok()?.to_string())
        .parent()
        .unwrap_or(std::path::Path::new("/"))
        .join("var/lib/pacman/local");

    Some(std::fs::read_dir(dir).ok()?.count().saturating_sub(1))
}
fn snap() -> Option<usize> {
    #[cfg(unix)]
    let dir = std::path::Path::new("/var/lib/snapd/snaps");
    #[cfg(windows)]
    let dir = std::path::Path::new(&std::env::var("MSYSTEM_PREFIX").ok()?.to_string())
        .parent()
        .unwrap_or(std::path::Path::new("/"))
        .join("var/lib/snapd/snaps");

    Some(std::fs::read_dir(dir).ok()?.count().saturating_sub(1))
}

fn scoop() -> Option<usize> {
    let home_dir = dirs::home_dir()?;
    let dir = home_dir.join("scoop").join("apps");

    Some(std::fs::read_dir(dir).ok()?.count().saturating_sub(1))
}

fn dpkg() -> Option<usize> {
    #[cfg(unix)]
    let dir = std::path::Path::new("/var/lib/dpkg/status");
    #[cfg(windows)]
    let dir = std::path::Path::new(&std::env::var("MSYSTEM_PREFIX").ok()?.to_string())
        .parent()
        .unwrap_or(std::path::Path::new("/"))
        .join("var/lib/dpkg/status");
    let file = std::fs::read_to_string(dir).ok()?;
    let mut package_count = 0;
    for line in file.lines() {
        if line.starts_with("Package:") {
            package_count += 1;
        }
    }
    Some(package_count)
}
pub async fn get_packages() -> Option<Packages> {
    tokio::task::spawn_blocking(|| {
        Some(Packages {
            snap: snap().unwrap_or_default(),
            dpkg: dpkg().unwrap_or_default(),
            pacman: pacman().unwrap_or_default(),
            scoop: scoop().unwrap_or_default(),
        })
    })
    .await
    .ok()?
}
