#[cfg(target_os = "windows")]
pub async fn get_locale() -> crate::error::Result<String> {
    use serde::Deserialize;

    use crate::{platform::wmi_query, share::detect_locale};

    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct Win32OperatingSystem {
        locale: String,
    }

    let results: Vec<Win32OperatingSystem> = wmi_query().await?;
    let hex = results.first().ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("Locale information not found")
    })?;

    let locale = detect_locale(&hex.locale).ok_or_else(|| {
        crate::error::NeofetchError::parse_error("locale", "Failed to detect locale")
    })?;
    Ok(locale)
}

#[cfg(target_os = "android")]
pub async fn get_locale() -> crate::error::Result<String> {
    crate::share::get_property("persist.sys.locale").ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("Locale property not available")
    })
}

#[cfg(any(target_os = "macos", target_os = "linux",))]
pub async fn get_locale() -> crate::error::Result<String> {
    std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .map_err(|_| {
            crate::error::NeofetchError::data_unavailable("Locale environment variables not set")
        })
}
