#[derive(Debug, Clone)]
pub struct Packages {
    snap: Vec<String>,
    dpkg: Vec<String>,
    pacman: Vec<String>,
}

impl Display for Packages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = Vec::new();
        if !self.dpkg.is_empty() {
            v.push(format!("{} (dpkg)", self.dpkg.len()))
        }
        if !self.snap.is_empty() {
            v.push(format!("{} (snap)", self.snap.len()))
        }
        if !self.pacman.is_empty() {
            v.push(format!("{} (pacman)", self.pacman.len()))
        }

        f.write_str(&v.join(", "))
    }
}

use std::fmt::Display;

use crate::share::exec_async;
use tracing::instrument;
#[instrument]
pub async fn get_packages() -> Option<Packages> {
    let (snap, dpkg, pacman) = tokio::join! {
      exec_async("snap", ["list"]),
      exec_async("dpkg", ["-l"]),
      exec_async("pacman", ["-Q"]),
    };

    let snap: Vec<String> = if let Some(s) = snap {
        s.lines().skip(1).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    let dpkg: Vec<String> = if let Some(s) = dpkg {
        s.lines().skip(5).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    let pacman: Vec<String> = if let Some(s) = pacman {
        s.lines().skip(5).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    Some(Packages { snap, dpkg, pacman })
}
