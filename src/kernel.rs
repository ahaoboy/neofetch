use crate::share::exec;

#[cfg(windows)]
pub fn get_kernel() -> Option<String> {
    let s = exec("wmic", ["os", "get", "Version"]).or(exec(
        "powershell",
        [
            "-c",
            "Get-CimInstance Win32_OperatingSystem | Select-Object Version",
        ],
    ))?;
    s.trim()
        .lines()
        .last()?
        .replace("Version", "")
        .trim()
        .to_string()
        .into()
}
#[cfg(unix)]
pub fn get_kernel() -> Option<String> {
    let s = exec("uname", ["-r"])?;
    s.trim().to_string().into()
}
