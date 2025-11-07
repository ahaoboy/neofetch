use which_shell::ShellVersion;

use crate::error::Result;

/// Retrieves the current shell information
///
/// Returns the shell version information. Returns an error if the shell
/// cannot be determined or if the task fails.
pub async fn which_shell() -> Result<ShellVersion> {
    let result = tokio::task::spawn_blocking(which_shell::which_shell).await?;
    result.ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("Shell information not available")
    })
}
