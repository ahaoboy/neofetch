use tracing::instrument;

use crate::share::{get_pid_name, get_ppid};
#[instrument]
pub async fn get_terminal() -> Option<String> {
    if std::env::var("WT_SESSION").is_ok() {
        return Some("Windows Terminal".into());
    }

    let mut id = std::process::id();

    while let Some(pid) = get_ppid(id) {
        id = pid;
        if let Some(name) = get_pid_name(id) {
            match name.as_str() {
                "gnome-terminal-" => return Some("gnome-terminal".into()),
                "urxvtd" => return Some("urxvtd".into()),
                "nvim" => return Some("Neovim Terminal".into()),
                "NeoVimServer" => return Some("VimR Terminal".into()),
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
