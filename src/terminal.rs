use crate::share::{get_pid_name, get_ppid};

#[derive(Debug, Clone)]
pub struct Terminal {
    pub name: String,
    pub version: Option<String>,
}

fn detect_terminal_type(mut id: u32) -> Option<String> {
    while let Some(pid) = get_ppid(id) {
        id = pid;
        if let Some(name) = get_pid_name(id) {
            match name.as_str() {
                "gnome-terminal-" => return Some("gnome-terminal".into()),
                "urxvtd" => return Some("urxvtd".into()),
                "nvim" => return Some("Neovim Terminal".into()),
                "NeoVimServer" => return Some("VimR Terminal".into()),
                "node" => return Some("node".into()),
                "ruby" | "1" | "tmux" | "systemd" | "sshd" | "python" | "USER" | "PID"
                | "kdeinit" | "launchd" | "bwrap" => break,
                _ => {}
            }
        } else {
            break;
        }
    }
    None
}

pub async fn get_terminal() -> crate::error::Result<Terminal> {
    if std::env::var("WT_SESSION").is_ok() {
        return Ok(Terminal {
            name: "Windows Terminal".into(),
            version: None,
        });
    }

    if let Ok(name) = std::env::var("TERM_PROGRAM") {
        return Ok(Terminal {
            name,
            version: std::env::var("TERM_PROGRAM_VERSION").ok(),
        });
    }

    if std::env::var("TERMUX_VERSION").is_ok() {
        return Ok(Terminal {
            name: "Termux".into(),
            version: None,
        });
    }

    if let Some(name) = detect_terminal_type(std::process::id()) {
        return Ok(Terminal {
            name,
            version: None,
        });
    }

    if let Ok(name) = std::env::var("TERM") {
        return Ok(Terminal {
            name,
            version: None,
        });
    }

    Err(crate::error::NeofetchError::data_unavailable(
        "Terminal information not available",
    ))
}

impl std::fmt::Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.version {
            f.write_str(&format!("{} ({})", self.name, v))
        } else {
            f.write_str(&self.name)
        }
    }
}
