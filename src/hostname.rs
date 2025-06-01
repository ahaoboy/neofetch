use crate::share::wmi_query;

pub async fn get_hostname() -> Option<String> {
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
