#[cfg(windows)]
pub async fn get_host() -> crate::error::Result<String> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_computersystem")]
    struct Computersystem {
        #[serde(rename = "Manufacturer")]
        manufacturer: String,
    }

    let results: Vec<Computersystem> = wmi_query().await?;
    results
        .first()
        .map(|i| i.manufacturer.clone())
        .ok_or_else(|| crate::error::NeofetchError::data_unavailable("Host information not found"))
}

#[cfg(not(target_os = "android"))]
pub async fn get_rom() -> crate::error::Result<String> {
    Err(crate::error::NeofetchError::UnsupportedPlatform)
}

#[cfg(not(target_os = "android"))]
pub async fn get_baseband() -> crate::error::Result<String> {
    Err(crate::error::NeofetchError::UnsupportedPlatform)
}

#[cfg(target_os = "android")]
pub async fn get_rom() -> crate::error::Result<String> {
    crate::share::get_property("ro.build.display.id").ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("ROM information not available")
    })
}

#[cfg(target_os = "android")]
pub async fn get_baseband() -> crate::error::Result<String> {
    crate::share::get_property("ro.baseband").ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("Baseband information not available")
    })
}

#[cfg(unix)]
pub async fn get_host() -> crate::error::Result<String> {
    let name_result = tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/board_name").await;
    let version_result =
        tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/product_version").await;

    if let (Ok(name), Ok(version)) = (name_result, version_result) {
        if !name.is_empty() && !version.is_empty() {
            return Ok(format!("{} {}", name.trim(), version.trim()));
        }
    }

    Err(crate::error::NeofetchError::data_unavailable(
        "Host information not available",
    ))
}
