#[cfg(windows)]
pub async fn get_host() -> crate::error::Result<String> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_computersystem")]
    struct Computersystem {
        #[serde(rename = "Manufacturer")]
        manufacturer: String,
        #[serde(rename = "Model")]
        model: String,
    }

    let results: Vec<Computersystem> = wmi_query().await?;
    results
        .first()
        .map(|i| {
            let mfr = i.manufacturer.trim();
            let mdl = i.model.trim();
            if mdl.is_empty() || mdl.eq_ignore_ascii_case("System Product Name") {
                mfr.to_string()
            } else if mdl.to_lowercase().contains(&mfr.to_lowercase()) {
                mdl.to_string()
            } else {
                format!("{} {}", mfr, mdl)
            }
        })
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
    crate::share::get_property("ro.build.display.id")
}

#[cfg(target_os = "android")]
pub async fn get_baseband() -> crate::error::Result<String> {
    crate::share::get_property("ro.baseband")
}

#[cfg(target_os = "macos")]
pub async fn get_host() -> crate::error::Result<String> {
    crate::platform::macos::get_host_model().await
}

#[cfg(target_os = "android")]
pub async fn get_host() -> crate::error::Result<String> {
    let manufacturer = crate::share::get_property("ro.product.manufacturer").unwrap_or_default();
    let model = crate::share::get_property("ro.product.model").unwrap_or_default();

    if manufacturer.is_empty() && model.is_empty() {
        return Err(crate::error::NeofetchError::data_unavailable(
            "Host information not available",
        ));
    }

    // Skip manufacturer if model already contains it
    if !model.is_empty() && model.to_lowercase().contains(&manufacturer.to_lowercase()) {
        return Ok(model);
    }

    Ok(format!("{} {}", manufacturer, model).trim().to_string())
}

#[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
pub async fn get_host() -> crate::error::Result<String> {
    // Try sys_vendor + product_name (best combo)
    let vendor = tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor").await;
    let product = tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name").await;

    if let (Ok(v), Ok(p)) = (&vendor, &product)
        && !v.trim().is_empty()
        && !p.trim().is_empty()
    {
        let vendor = v.trim();
        let product = p.trim();
        // Skip vendor if product already contains it
        if product.to_lowercase().contains(&vendor.to_lowercase()) {
            return Ok(product.to_string());
        }
        return Ok(format!("{} {}", vendor, product));
    }

    // Fallback: product_name alone
    if let Ok(p) = &product
        && !p.trim().is_empty()
    {
        return Ok(p.trim().to_string());
    }

    // Fallback: board_name + product_version
    let board = tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/board_name").await;
    let version = tokio::fs::read_to_string("/sys/devices/virtual/dmi/id/product_version").await;

    match (&board, &version) {
        (Ok(b), Ok(v)) if !b.trim().is_empty() && !v.trim().is_empty() => {
            Ok(format!("{} {}", b.trim(), v.trim()))
        }
        (Ok(b), _) if !b.trim().is_empty() => Ok(b.trim().to_string()),
        _ => Err(crate::error::NeofetchError::data_unavailable(
            "Host information not available",
        )),
    }
}
