use crate::share::{exec, get_file_name};

#[derive(Debug, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Pwsh,
    Cmd,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ShellVersion {
    shell: Shell,
    version: String,
}

impl ToString for ShellVersion {
    fn to_string(&self) -> String {
        format!("{} {}", self.shell.to_string(), self.version)
    }
}

impl From<&str> for Shell {
    fn from(val: &str) -> Self {
        match val {
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            "OpenConsole" => Shell::PowerShell,
            "powershell" => Shell::PowerShell,
            "bash" => Shell::Bash,
            "pwsh" => Shell::Pwsh,
            "cmd" => Shell::Cmd,
            _ => Shell::Unknown,
        }
    }
}

impl ToString for Shell {
    fn to_string(&self) -> String {
        match self {
            Shell::Fish => "fish".into(),
            Shell::Zsh => "zsh".into(),
            Shell::Bash => "bash".into(),
            Shell::PowerShell => "powershell".into(),
            Shell::Cmd => "cmd".into(),
            Shell::Pwsh => "pwsh".into(),
            Shell::Unknown => "unknown".into(),
        }
    }
}

pub fn get_shell_version(sh: Shell) -> Option<String> {
    let version = exec(sh.to_string().as_str(), ["--version"])?;
    match sh {
        Shell::Fish => {
            // fish, version 3.6.1
            return Some(version[14..].trim().into());
        }
        Shell::Pwsh => {
            // PowerShell 7.4.1
            return Some(version[11..].trim().into());
        }
        Shell::Cmd => {
            // Microsoft Windows [版本 10.0.22635.2700]
            // (c) Microsoft Corporation。保留所有权利。
            let s = version
                .lines()
                .next()?
                .split(' ')
                .last()?
                .split(']')
                .next()?
                .trim();
            Some(s.into())
        }
        _ => Some("".into()),
    }
}

#[cfg(windows)]
pub fn get_shell() -> Option<ShellVersion> {
    let list = exec("wmic", ["process", "get", "ExecutablePath"])?;
    let list = list.trim().lines().rev().filter(|i| i.trim().len() > 0);

    for path in list {
        let cmd = get_file_name(path)?;
        let shell: Shell = cmd.as_str().into();
        match shell {
            Shell::Unknown => {
                continue;
            }
            _ => {
                let version = get_shell_version(shell).unwrap_or_default();
                return Some(ShellVersion { shell, version });
            }
        }
    }

    None
}

#[cfg(unix)]
pub fn get_shell() -> Option<ShellVersion> {
    if let Ok(sh) = std::env::var("SHELL") {
        let name = get_file_name(&sh)?;
        let shell: Shell = name.as_str().into();
        let version = get_shell_version(shell).unwrap_or_default();

        return ShellVersion { shell, version }.into();
    }

    None
}
