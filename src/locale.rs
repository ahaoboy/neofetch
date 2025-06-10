#[cfg(target_os = "windows")]
pub async fn get_locale() -> Option<String> {
    use serde::Deserialize;

    use crate::share::{detect_locale, wmi_query};
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct Win32OperatingSystem {
        locale: String,
    }

    let results: Vec<Win32OperatingSystem> = wmi_query().await?;
    let hex = results.first()?;

    detect_locale(&hex.locale)
}

#[cfg(target_os = "android")]
pub async fn get_locale() -> Option<String> {
    crate::share::get_property("persist.sys.locale")
}
