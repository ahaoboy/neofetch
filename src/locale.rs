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

    let locale = detect_locale(&hex.locale)?;
    Ok(locale)
}

#[cfg(target_os = "android")]
pub async fn get_locale() -> crate::error::Result<String> {
    crate::share::get_property("persist.sys.locale")
}

#[cfg(any(target_os = "macos", target_os = "linux",))]
pub async fn get_locale() -> crate::error::Result<String> {
    let locale = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_default();

    // "C" / "C.UTF-8" is the fallback locale — try harder to find the real one
    if locale.starts_with('C') || locale.is_empty() {
        // Try $LANGUAGE (GNU extension: "en_US:en"), often set even when LANG=C
        if let Ok(lang) = std::env::var("LANGUAGE") {
            if let Some(first) = lang.split(':').next() {
                if !first.is_empty() && !first.starts_with('C') {
                    let real = if first.contains('.') {
                        first.to_string()
                    } else {
                        format!("{}.UTF-8", first)
                    };
                    return Ok(real);
                }
            }
        }

        // Try /etc/default/locale (Ubuntu/Debian) or /etc/locale.conf (Arch)
        for path in ["/etc/default/locale", "/etc/locale.conf"] {
            if let Ok(content) = tokio::fs::read_to_string(path).await {
                for line in content.lines() {
                    if let Some(val) = line
                        .strip_prefix("LANG=")
                        .or_else(|| line.strip_prefix("LC_ALL="))
                    {
                        let val = val.trim_matches('"').trim();
                        if !val.is_empty() && !val.starts_with('C') {
                            return Ok(val.to_string());
                        }
                    }
                }
            }
        }
    }

    if locale.is_empty() {
        return Err(crate::error::NeofetchError::data_unavailable(
            "Locale environment variables not set",
        ));
    }
    Ok(locale)
}
