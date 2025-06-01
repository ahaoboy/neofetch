use human_bytes::human_bytes;
use std::fmt::Display;

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

#[cfg(windows)]
pub async fn get_disk() -> Option<Vec<Disk>> {
    use crate::share::wmi_query;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_logicaldisk")]
    struct Logicaldisk {
        #[serde(rename = "DeviceID")]
        device_id: String,
        #[serde(rename = "FreeSpace")]
        free_space: u64,
        #[serde(rename = "Size")]
        size: u64,
    }

    let results: Vec<Logicaldisk> = wmi_query().await?;
    let mut v = vec![];
    for i in results {
        v.push(Disk {
            name: i.device_id,
            total: i.size,
            used: i.size - i.free_space,
        })
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
