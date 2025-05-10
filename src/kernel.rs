use tracing::instrument;

#[cfg(windows)]
#[instrument]
pub async fn get_kernel() -> Option<String> {
    use crate::share::exec_async;

    let s = exec_async("wmic", ["os", "get", "Version"])
        .await
        .or(exec_async(
            "powershell",
            [
                "-c",
                "Get-CimInstance Win32_OperatingSystem | Select-Object Version",
            ],
        )
        .await)?;
    s.trim()
        .lines()
        .last()?
        .replace("Version", "")
        .trim()
        .to_string()
        .into()
}
#[cfg(unix)]
#[instrument]
pub fn get_kernel() -> Option<String> {
    let s = exec("uname", ["-r"])?;
    s.trim().to_string().into()
}
