use tracing::instrument;

use crate::share::exec_async;
#[instrument]
pub async fn get_user() -> Option<String> {
    if let Some(s) = exec_async("id", ["-un"]).await {
        return Some(s);
    }

    if let Ok(s) = std::env::var("username") {
        return Some(s);
    }

    if let Ok(s) = std::env::var("HOME") {
        let name = s.replace('\\', "/");
        let name = name.split('/').next_back()?;
        return Some(name.into());
    }
    Some("".into())
}
