#[cfg(target_os = "windows")]
pub async fn get_locale() -> Option<String> {
    use serde::Deserialize;

    use crate::{platform::wmi_query, share::detect_locale};
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct Win32OperatingSystem {
        locale: String,
    }

    let results: Vec<Win32OperatingSystem> = wmi_query().await.ok()?;
    let hex = results.first()?;

    detect_locale(&hex.locale)
}

#[cfg(target_os = "android")]
pub async fn get_locale() -> Option<String> {
    crate::share::get_property("persist.sys.locale")
}

#[cfg(any(target_os = "macos", target_os = "linux",))]
pub async fn get_locale() -> Option<String> {
    std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .ok()
}
