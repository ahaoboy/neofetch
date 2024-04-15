use crate::share::exec;

pub fn get_memory() -> Option<String> {
    let total = exec(
        "awk",
        ["-F", ":", "/MemTotal/ {printf $2; exit}", "/proc/meminfo"],
    )?;
    let total = total.trim().split(' ').next()?;
    let total: f64 = total.parse().ok()?;

    let free = exec(
        "awk",
        ["-F", ":", "/MemFree/ {printf $2; exit}", "/proc/meminfo"],
    )?;
    let free = free.trim().split(' ').next()?;
    let free: f64 = free.parse().ok()?;
    use human_bytes::human_bytes;

    Some(format!(
        "{} / {}",
        human_bytes(free * 1024.),
        human_bytes(total * 1024.),
    ))
}
