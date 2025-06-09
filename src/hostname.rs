#[cfg(windows)]
pub async fn get_hostname() -> Option<String> {
    use crate::share::wmi_query;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "Name")]
        name: String,
    }
    let results: Vec<ComputerSystem> = wmi_query().await?;

    results.first().map(|i| i.name.clone())
}
#[cfg(not(windows))]
pub async fn get_hostname() -> Option<String> {
    use std::ffi::CStr;
    let mut buffer = vec![0u8; 256];

    let result = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut std::ffi::c_char, buffer.len()) };

    if result != 0 {
        return None;
    }

    let hostname = unsafe {
        CStr::from_ptr(buffer.as_ptr() as *const std::ffi::c_char)
            .to_string_lossy()
            .into_owned()
    };

    Some(hostname)
}
