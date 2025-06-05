#[cfg(windows)]
pub async fn get_hostname() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "Name")]
        name: String,
    }
    let results: Vec<ComputerSystem> = wmi_query().await?;

    results.first().map(|i| i.name.clone())
}
#[cfg(not(windows))]
pub async fn get_hostname() -> Option<String> {
    use crate::share::exec_async;
    let s = exec_async("hostname", []).await?;
    s.into()
}
