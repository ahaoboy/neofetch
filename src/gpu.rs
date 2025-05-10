use tracing::instrument;

#[cfg(windows)]
#[instrument]
pub async fn get_gpu() -> Option<String> {
    use crate::share::exec_async;

    let s = exec_async("wmic", ["path", "Win32_VideoController", "get", "caption"]).await.or(exec_async(
        "powershell",
        [
            "-c",
            "Get-CimInstance Win32_VideoController | Select-Object caption",
        ],
    ).await)?;
    s.lines()
        .last()?
        .replace("Microsoft ", "")
        .trim()
        .to_string()
        .into()
}
#[cfg(unix)]
#[instrument]
pub fn get_gpu() -> Option<String> {
    use regex::Regex;

    if let Some(s) = exec("lspci", ["-mm"]) {
        let reg = Regex::new("\"(.*?)\" \"(.*?)\" \"(.*?)\"").unwrap();

        for line in s.lines() {
            let cap = reg.captures(line)?;
            if let (Some(_), Some(a), Some(b)) = (cap.get(1), cap.get(2), cap.get(3)) {
                if ["Display", "3D", "VGA"]
                    .into_iter()
                    .any(|i| b.as_str().contains(i))
                {
                    return Some(a.as_str().to_string() + b.as_str());
                }
            }
        }
    }
    None
}
