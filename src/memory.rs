use crate::share::wmi_query;

pub async fn get_memory() -> Option<String> {
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
