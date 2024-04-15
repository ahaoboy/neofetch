#[derive(Debug, Clone)]
pub struct Packages {
    snap: Vec<String>,
    dpkg: Vec<String>,
    pacman: Vec<String>,
}

impl ToString for Packages {
    fn to_string(&self) -> String {
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

        v.join(", ")
    }
}

use crate::share::exec;

pub fn get_packages() -> Option<Packages> {
    let snap: Vec<String> = if let Some(s) = exec("snap", ["list"]) {
        s.lines().skip(1).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    let dpkg: Vec<String> = if let Some(s) = exec("dpkg", ["-l"]) {
        s.lines().skip(5).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    let pacman: Vec<String> = if let Some(s) = exec("pacman", ["-Q"]) {
        s.lines().skip(5).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    };
    Some(Packages { snap, dpkg, pacman })
}
