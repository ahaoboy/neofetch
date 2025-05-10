use tracing::instrument;

#[cfg(windows)]
#[instrument]
pub async fn get_battery() -> Option<String> {
    use crate::share::exec_async;
    let s = exec_async(
        "wmic",
        ["Path", "Win32_Battery", "get", "EstimatedChargeRemaining"],
    )
    .await?;

    if s.lines().count() < 2 {
        return None;
    }
    Some(s.lines().last()?.to_string())
}

#[cfg(unix)]
#[instrument]
pub fn get_battery() -> Option<String> {
    None
}
