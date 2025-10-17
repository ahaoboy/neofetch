#[cfg(not(windows))]
pub async fn get_memory() -> Option<String> {
    let s = tokio::fs::read_to_string("/proc/meminfo").await.ok()?;

    let mut total = Some("");
    let mut free = Some("");

    let total_header = "MemTotal:";
    let free_header = "MemFree:";
    for line in s.lines() {
        if let Some(line) = line.strip_prefix(total_header) {
            total = line.trim().split(" ").next();
        }
        if let Some(line) = line.strip_prefix(free_header) {
            free = line.trim().split(" ").next();
        }
    }
    let total = total?.trim().split(' ').next()?;
    let total: f64 = total.parse().ok()?;

    let free = free?.trim().split(' ').next()?;
    let free: f64 = free.parse().ok()?;
    use human_bytes::human_bytes;

    Some(format!(
        "{} / {}",
        human_bytes(free * 1024.),
        human_bytes(total * 1024.),
    ))
}

#[cfg(windows)]
pub async fn get_memory() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "TotalVisibleMemorySize")]
        total_visible_memory_size: u64,
        #[serde(rename = "FreePhysicalMemory")]
        free_physical_memory: u64,
    }
    let results: Vec<OperatingSystem> = wmi_query().await?;
    let info = results.first()?;

    let used = (info.total_visible_memory_size - info.free_physical_memory) as f64;
    let total = info.total_visible_memory_size as f64;
    use human_bytes::human_bytes;

    Some(format!(
        "{} / {} ({}%)",
        human_bytes(used * 1024.),
        human_bytes(total * 1024.),
        (used / total * 100.) as u32
    ))
}
