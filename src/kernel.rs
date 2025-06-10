#[cfg(windows)]
pub async fn get_kernel() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "Version")]
        version: String,
    }
    let results: Vec<OperatingSystem> = wmi_query().await?;
    let os = results.first()?.version.clone();
    let mut v = vec![os];
    use winreg::RegKey;
    use winreg::enums::*;
    if let Ok(subkey) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .and_then(|op| op.get_value::<String, _>("DisplayVersion"))
    {
        v.push(format!("({subkey})"));
    }

    Some(v.join(" "))
}
#[cfg(unix)]
pub async fn get_kernel() -> Option<String> {
    use std::ffi::CStr;
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };

    let result = unsafe { libc::uname(&mut uts) };
    if result != 0 {
        return None;
    }
    let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    Some(release)
}
