#[cfg(windows)]
pub async fn get_user() -> Option<String> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "UserName")]
        user_name: Option<String>,
    }
    let results: Vec<ComputerSystem> = wmi_query().await.ok()?;
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

    if !username.is_empty() {
        return Some(username);
    }
    if let Ok(s) = std::env::var("username") {
        return Some(s);
    }

    if let Ok(s) = std::env::var("HOME") {
        let name = s.replace('\\', "/");
        let name = name.split('/').next_back()?;
        return Some(name.into());
    }
    None
}
