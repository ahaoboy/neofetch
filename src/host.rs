#[cfg(windows)]
pub async fn get_host() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_computersystem")]
    struct Computersystem {
        #[serde(rename = "Manufacturer")]
        manufacturer: String,
    }

    let results: Vec<Computersystem> = wmi_query().await?;
    results.first().map(|i| i.manufacturer.clone())
}

#[cfg(not(target_os = "android"))]
pub async fn get_rom() -> Option<String> {
    None
}

#[cfg(not(target_os = "android"))]
pub async fn get_baseband() -> Option<String> {
    None
}

#[cfg(target_os = "android")]
pub async fn get_rom() -> Option<String> {
    crate::share::get_property("ro.build.display.id")
}

#[cfg(target_os = "android")]
pub async fn get_baseband() -> Option<String> {
    crate::share::get_property("ro.baseband")
}

#[cfg(unix)]
pub async fn get_host() -> Option<String> {
    if let (Ok(name), Ok(version)) = (
        tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/board_name").await,
        tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/product_version").await,
    ) && !name.is_empty()
        && !version.is_empty()
    {
        return format!("{} {}", name.trim(), version.trim()).into();
    }
    None
}
