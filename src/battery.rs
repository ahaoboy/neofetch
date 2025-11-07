#[cfg(windows)]
pub async fn get_battery() -> crate::error::Result<u32> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_Battery")]
    struct Battery {
        #[serde(rename = "EstimatedChargeRemaining")]
        estimated_charge_remaining: u32,
    }

    let results: Vec<Battery> = wmi_query().await?;
    results
        .first()
        .map(|i| i.estimated_charge_remaining)
        .ok_or_else(|| {
            crate::error::NeofetchError::data_unavailable("Battery information not available")
        })
}

#[cfg(unix)]
pub async fn get_battery() -> crate::error::Result<u32> {
    Err(crate::error::NeofetchError::UnsupportedPlatform)
}
