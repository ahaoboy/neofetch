#[cfg(windows)]
pub async fn get_user() -> crate::error::Result<String> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "UserName")]
        user_name: Option<String>,
    }

    let results: Vec<ComputerSystem> = wmi_query().await?;
    let name = results
        .first()
        .and_then(|i| i.user_name.clone())
        .ok_or_else(|| crate::error::NeofetchError::data_unavailable("User name not found"))?;

    let username = name
        .split("\\")
        .last()
        .map(|i| i.to_owned())
        .ok_or_else(|| {
            crate::error::NeofetchError::parse_error("username", "Invalid username format")
        })?;

    Ok(username)
}

#[cfg(unix)]
pub async fn get_user() -> crate::error::Result<String> {
    use std::ffi::CStr;

    let uid = unsafe { libc::getuid() };

    let passwd = unsafe { libc::getpwuid(uid) };
    if passwd.is_null() {
        return Err(crate::error::NeofetchError::system_call(
            "Failed to get user information from getpwuid",
        ));
    }

    let username = unsafe {
        CStr::from_ptr((*passwd).pw_name)
            .to_string_lossy()
            .into_owned()
    };

    if !username.is_empty() {
        return Ok(username);
    }

    // Fallback to environment variables
    if let Ok(s) = std::env::var("username") {
        return Ok(s);
    }

    if let Ok(s) = std::env::var("HOME") {
        let name = s.replace('\\', "/");
        if let Some(name) = name.split('/').next_back() {
            return Ok(name.into());
        }
    }

    Err(crate::error::NeofetchError::data_unavailable(
        "User information not available",
    ))
}
