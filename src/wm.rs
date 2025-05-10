use tracing::instrument;

use crate::share::exec_async;
#[instrument]
pub async fn get_wm() -> Option<String> {
    
    let s = exec_async("tasklist", []).await?;

    for i in s.lines() {
        if i.starts_with("explorer.exe") {
            return Some("Explorer".into());
        }
    }
    s.into()
}
#[instrument]
pub async fn get_wm_theme() -> Option<String> {
    use crate::share::get_file_name;
    let s = exec_async("head", ["-n1", "/proc/registry/HKEY_CURRENT_USER/Software/Microsoft/Windows/CurrentVersion/Themes/CurrentTheme"])  .await?;
    let s = get_file_name(&s)?;
    Some(s)
}
