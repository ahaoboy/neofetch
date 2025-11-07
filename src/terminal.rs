use crate::share::{get_pid_name, get_ppid};

/// Retrieves the terminal emulator name
///
/// Attempts to identify the terminal emulator by checking environment variables
/// and traversing the process tree to find known terminal process names.
pub async fn get_terminal() -> crate::error::Result<String> {
    if std::env::var("WT_SESSION").is_ok() {
        return Ok("Windows Terminal".into());
    }

    let mut id = std::process::id();

    while let Some(pid) = get_ppid(id) {
        id = pid;
        if let Some(name) = get_pid_name(id) {
            match name.as_str() {
                "gnome-terminal-" => return Ok("gnome-terminal".into()),
                "urxvtd" => return Ok("urxvtd".into()),
                "nvim" => return Ok("Neovim Terminal".into()),
                "NeoVimServer" => return Ok("VimR Terminal".into()),
                "node" => return Ok("node".into()),
                "ruby" | "1" | "tmux" | "systemd" | "sshd" | "python" | "USER" | "PID"
                | "kdeinit" | "launchd" | "bwrap" => break,
                _ => {}
            }
        } else {
            break;
        }
    }
    Err(crate::error::NeofetchError::data_unavailable(
        "Terminal information not available",
    ))
}
