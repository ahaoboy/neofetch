use crate::error::Result;

pub async fn get_terminal() -> Result<which_terminal::TerminalInfo> {
    let result = tokio::task::spawn_blocking(which_terminal::which_terminal).await?;
    result.ok_or_else(|| {
        crate::error::NeofetchError::data_unavailable("Shell information not available")
    })
}
