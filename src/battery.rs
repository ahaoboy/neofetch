#[cfg(windows)]
pub fn get_battery() -> Option<String> {
    use crate::share::exec;
    let s = exec(
        "wmic",
        ["Path", "Win32_Battery", "get", "EstimatedChargeRemaining"],
    )?;

    if s.lines().count() < 2 {
        return None;
    }
    Some(s.lines().last()?.to_string())
}

#[cfg(unix)]
pub fn get_battery() -> Option<String> {
    None
}
