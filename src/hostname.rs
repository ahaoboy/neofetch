use crate::share::exec_async;
use tracing::instrument;
#[instrument]
pub async fn get_hostname() -> Option<String> {
    let s = exec_async("hostname", []).await?;
    s.into()
}
