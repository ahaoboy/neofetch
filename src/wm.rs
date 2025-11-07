#[cfg(windows)]
pub async fn get_wm() -> crate::error::Result<String> {
    tokio::task::spawn_blocking(|| -> crate::error::Result<String> {
        use windows::Win32::{
            Foundation::CloseHandle,
            System::Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
                TH32CS_SNAPPROCESS,
            },
        };

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).map_err(|e| {
                crate::error::NeofetchError::system_call(format!(
                    "Failed to create process snapshot: {}",
                    e
                ))
            })?;

            if snapshot.is_invalid() {
                return Err(crate::error::NeofetchError::system_call(
                    "Invalid process snapshot",
                ));
            }

            let mut pe32 = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };

            if Process32FirstW(snapshot, &mut pe32).is_ok() {
                loop {
                    let name = String::from_utf16_lossy(
                        &pe32.szExeFile[..pe32.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)],
                    );

                    if name.starts_with("explorer.exe") {
                        let _ = CloseHandle(snapshot);
                        return Ok("Explorer".into());
                    }

                    if Process32NextW(snapshot, &mut pe32).is_err() {
                        break;
                    }
                }
            }
            let _ = CloseHandle(snapshot);
        }

        Err(crate::error::NeofetchError::data_unavailable(
            "Window manager not found",
        ))
    })
    .await?
}

#[cfg(not(windows))]
pub async fn get_wm() -> crate::error::Result<String> {
    Err(crate::error::NeofetchError::UnsupportedPlatform)
}

#[cfg(windows)]
pub async fn get_wm_theme() -> crate::error::Result<String> {
    tokio::task::spawn_blocking(|| -> crate::error::Result<String> {
        use crate::share::get_file_name;

        use winreg::RegKey;
        use winreg::enums::*;

        let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);

        let themes_key = hkey_current_user
            .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes")
            .map_err(|e| {
                crate::error::NeofetchError::system_call(format!(
                    "Failed to open registry key: {}",
                    e
                ))
            })?;

        let current_theme: String = themes_key.get_value("CurrentTheme").map_err(|e| {
            crate::error::NeofetchError::system_call(format!(
                "Failed to read CurrentTheme value: {}",
                e
            ))
        })?;

        let theme_name = get_file_name(&current_theme).ok_or_else(|| {
            crate::error::NeofetchError::parse_error("theme_path", "Failed to extract theme name")
        })?;

        Ok(theme_name)
    })
    .await?
}

#[cfg(not(windows))]
pub async fn get_wm_theme() -> crate::error::Result<String> {
    Err(crate::error::NeofetchError::UnsupportedPlatform)
}
