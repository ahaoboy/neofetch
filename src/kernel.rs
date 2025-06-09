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
    results.first().map(|i| i.version.clone())
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
