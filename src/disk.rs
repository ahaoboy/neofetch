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
pub fn get_disk() -> Option<Disk> {
    use regex::Regex;
    let s = exec("wmic", ["logicaldisk", "get", "deviceid,freespace,size"])?;
    let s = s.lines().nth(1)?.trim().to_string();
    let re = Regex::new(r"([a-zA-Z0-9]+):?\s+([0-9]+)\s+([0-9]+)").ok()?;
    let cap = re.captures(&s)?;
    let name = cap.get(1).unwrap().as_str().to_string();
    let free = cap.get(2).unwrap().as_str().parse::<u64>().ok()?;
    let total = cap.get(3).unwrap().as_str().parse::<u64>().ok()?;
    let used = total - free;
    Some(Disk { name, used, total })
}

#[cfg(unix)]
pub fn get_disk() -> Option<Disk> {
    use regex::Regex;
    let s = exec("df", [])?;
    let mut disk = Disk {
        name: "".to_string(),
        total: 0,
        used: 0,
    };
    for i in s.lines().skip(1) {
        let re = Regex::new(r"([a-zA-Z0-9]+):?\s+([0-9]+)\s+([0-9]+)\s+([0-9]+).*?").ok()?;
        let cap = re.captures(i.trim())?;
        let name = cap.get(1).unwrap().as_str().to_string();
        let total = cap.get(2).unwrap().as_str().parse::<u64>().ok()? * 1024;
        let used = cap.get(3).unwrap().as_str().parse::<u64>().ok()? * 1024;

        if total > disk.total {
            disk.name = name;
            disk.total = total;
            disk.used = used;
        }
    }
    Some(disk)
}
