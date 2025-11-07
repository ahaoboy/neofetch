#[cfg(windows)]
pub async fn get_battery() -> Option<u32> {
    use serde::Deserialize;

    use crate::platform::wmi_query;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_Battery")]
    struct Battery {
        #[serde(rename = "EstimatedChargeRemaining")]
        estimated_charge_remaining: u32,
    }

    let results: Vec<Battery> = wmi_query().await.ok()?;
    results.first().map(|i| i.estimated_charge_remaining)
}

#[cfg(unix)]
pub async fn get_battery() -> Option<u32> {
    None
}
