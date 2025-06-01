pub async fn get_wm() -> Option<String> {
    tokio::task::spawn_blocking(|| {
        use windows::Win32::{
            Foundation::CloseHandle,
            System::Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
                TH32CS_SNAPPROCESS,
            },
        };

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;

            if snapshot.is_invalid() {
                return None;
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
                        return Some("Explorer".into());
                    }

                    if Process32NextW(snapshot, &mut pe32).is_err() {
                        break;
                    }
                }
            }
            CloseHandle(snapshot).ok()?;
        }

        None
    })
    .await
    .ok()?
}

pub async fn get_wm_theme() -> Option<String> {
    tokio::task::spawn_blocking(|| {
        use crate::share::get_file_name;

        use winreg::RegKey;
        use winreg::enums::*;

        let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);

        let themes_key = hkey_current_user
            .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Themes")
            .ok()?;

        let current_theme: String = themes_key.get_value("CurrentTheme").ok()?;

        let s = get_file_name(&current_theme)?;
        Some(s)
    })
    .await
    .ok()?
}
