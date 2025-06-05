use which_shell::ShellVersion;

pub async fn which_shell() -> Option<ShellVersion> {
    tokio::task::spawn_blocking(which_shell::which_shell)
        .await
        .ok()?
}
