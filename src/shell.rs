use std::fmt::Display;

use regex::Regex;

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

impl Display for ShellVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {}", self.shell, self.version))
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

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Shell::Fish => "fish",
            Shell::Zsh => "zsh",
            Shell::Bash => "bash",
            Shell::PowerShell => "powershell",
            Shell::Cmd => "cmd",
            Shell::Pwsh => "pwsh",
            Shell::Unknown => "unknown",
        };
        f.write_str(s)
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
        Shell::Bash => {
            // GNU bash, version 5.2.26(1)-release (aarch64-unknown-linux-android)
            let re = Regex::new(r"([0-9]+).([0-9]+).([0-9]+)").unwrap();
            let cap = re.captures(&version)?;

            if let (Some(a), Some(b), Some(c)) = (cap.get(1), cap.get(2), cap.get(3)) {
                return Some(format!("{}.{}.{}", a.as_str(), b.as_str(), c.as_str()));
            }
            None
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
    let list = list.trim().lines().rev().filter(|i| !i.trim().is_empty());

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
