#[cfg(windows)]
pub async fn get_hostname() -> crate::error::Result<String> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_ComputerSystem")]
    struct ComputerSystem {
        #[serde(rename = "Name")]
        name: String,
    }

    let results: Vec<ComputerSystem> = wmi_query().await?;

    results
        .first()
        .map(|i| i.name.clone())
        .ok_or_else(|| crate::error::NeofetchError::data_unavailable("Hostname not found"))
}

#[cfg(not(windows))]
pub async fn get_hostname() -> crate::error::Result<String> {
    use std::ffi::CStr;

    let mut buffer = vec![0u8; 256];

    let result =
        unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, buffer.len()) };

    if result != 0 {
        return Err(crate::error::NeofetchError::system_call(
            "Failed to get hostname from gethostname",
        ));
    }

    let hostname = unsafe {
        CStr::from_ptr(buffer.as_ptr() as *const libc::c_char)
            .to_string_lossy()
            .into_owned()
    };

    Ok(hostname)
}
