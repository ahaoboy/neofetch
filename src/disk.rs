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
        f.write_str(&format!("{used} / {total} ({percent:2.0}%)"))
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

// #[cfg(target_os = "android")]
// const DISK_SKIP: [&str; 2] = ["overlay", "/dev/block"];

// #[cfg(target_os = "macos")]
// const DISK_SKIP: [&str; 1] = ["devfs"];

#[cfg(unix)]
fn get_filesystem_info(path: &str) -> Option<Disk> {
    use std::ffi::CString;
    use std::io;

    let path_cstr =
        CString::new(path).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)).ok()?;

    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::statvfs(path_cstr.as_ptr(), &mut stat) };

    if result != 0 {
        return None;
    }

    let total = (stat.f_blocks * stat.f_frsize) as u64;
    let available = (stat.f_bavail * stat.f_frsize) as u64;

    Some(Disk {
        name: path.to_string(),
        total,
        used: total - available,
    })
}

#[cfg(unix)]
pub async fn get_disk() -> Option<Vec<Disk>> {
    let v = vec![get_filesystem_info("/")?];
    Some(v)
}
