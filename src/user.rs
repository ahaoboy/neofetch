#[cfg(windows)]
pub async fn get_user() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "UserName")]
        user_name: Option<String>,
    }
    let results: Vec<ComputerSystem> = wmi_query().await?;
    let name = results.first().map(|i| i.user_name.clone())??;

    name.split("\\").last().map(|i| i.to_owned())
}

#[cfg(unix)]
pub async fn get_user() -> Option<String> {
    use std::ffi::CStr;
    let uid = unsafe { libc::getuid() };

    let passwd = unsafe { libc::getpwuid(uid) };
    if passwd.is_null() {
        return None;
    }

    let username = unsafe {
        CStr::from_ptr((*passwd).pw_name)
            .to_string_lossy()
            .into_owned()
    };

    if username.is_empty() {
        return None;
    }
    if let Ok(s) = std::env::var("username") {
        return Some(s);
    }

    if let Ok(s) = std::env::var("HOME") {
        let name = s.replace('\\', "/");
        let name = name.split('/').next_back()?;
        return Some(name.into());
    }
    Some(username)
}
