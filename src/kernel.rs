#[cfg(windows)]
pub async fn get_kernel() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "Version")]
        version: String,
    }
    let results: Vec<OperatingSystem> = wmi_query().await?;
    results.first().map(|i| i.version.clone())
}
#[cfg(unix)]
pub async fn get_kernel() -> Option<String> {
    use crate::share::exec_async;

    let s = exec_async("uname", ["-r"]).await?;
    s.trim().to_string().into()
}
