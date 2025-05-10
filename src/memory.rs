use crate::share::exec_async;
use tracing::instrument;
#[instrument]
pub async fn get_memory() -> Option<String> {
    let total = exec_async(
        "awk",
        ["-F", ":", "/MemTotal/ {printf $2; exit}", "/proc/meminfo"],
    ).await?;
    let total = total.trim().split(' ').next()?;
    let total: f64 = total.parse().ok()?;

    let free = exec_async(
        "awk",
        ["-F", ":", "/MemFree/ {printf $2; exit}", "/proc/meminfo"],
    ).await?;
    let free = free.trim().split(' ').next()?;
    let free: f64 = free.parse().ok()?;
    use human_bytes::human_bytes;

    Some(format!(
        "{} / {}",
        human_bytes(free * 1024.),
        human_bytes(total * 1024.),
    ))
}
