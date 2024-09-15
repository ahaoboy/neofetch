#[derive(Debug, Clone)]
pub struct Disk {
    pub name: String,
    pub total: u64,
    pub used: u64,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let used = human_bytes(self.used as f64);
        let total = human_bytes(self.total as f64);
        let percent = (self.used as f64 / self.total as f64) * 100.;
        f.write_str(&format!("{used} / {total} ({:2.0}%)", percent))
    }
}

use std::fmt::Display;

use human_bytes::human_bytes;

use crate::share::exec;

#[cfg(windows)]
pub fn get_disk() -> Option<Vec<Disk>> {
    use regex::Regex;
    let s = exec("wmic", ["logicaldisk", "get", "deviceid,freespace,size"]).or(exec(
        "powershell",
        [
            "-c",
            "Get-CimInstance Win32_logicaldisk | Select-Object deviceid,freespace,size",
        ],
    ))?;
    let mut v = vec![];
    let re = Regex::new(r"([a-zA-Z0-9]+):?\s+([0-9]+)\s+([0-9]+)").ok()?;
    for line in s.lines() {
        let s = line.trim().to_string();
        if let Some(cap) = re.captures(&s) {
            let name = cap.get(1).unwrap().as_str().to_string();
            if let (Ok(free), Ok(total)) = (
                cap.get(2).unwrap().as_str().parse::<u64>(),
                cap.get(3).unwrap().as_str().parse::<u64>(),
            ) {
                let used = total - free;
                let disk = Disk { name, used, total };
                v.push(disk);
            }
        }
    }

    Some(v)
}

#[cfg(target_os = "linux")]
const DISK_SKIP: [&str; 1] = ["tmpfs"];

#[cfg(target_os = "android")]
const DISK_SKIP: [&str; 2] = ["overlay", "/dev/block"];

#[cfg(target_os = "macos")]
const DISK_SKIP: [&str; 1] = ["devfs"];

#[cfg(unix)]
pub fn get_disk() -> Option<Vec<Disk>> {
    use regex::Regex;
    let s = exec("df", [])?;

    let mut v = vec![];
    for line in s.lines().skip(1) {
        let re = Regex::new(r"([a-zA-Z0-9]+):?\s+([0-9]+)\s+([0-9]+)\s+([0-9]+).*?").ok()?;
        let cap = re.captures(line.trim())?;
        let name = cap.get(1).unwrap().as_str().to_string();

        if DISK_SKIP.iter().any(|i| line.starts_with(i)) {
            continue;
        }

        if let (Ok(total), Ok(used)) = (
            cap.get(2).unwrap().as_str().parse::<u64>(),
            cap.get(3).unwrap().as_str().parse::<u64>(),
        ) {
            let total = total * 1024;
            let used = used * 1024;
            v.push(Disk { name, used, total })
        }
    }
    Some(v)
}
