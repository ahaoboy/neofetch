pub fn get_wm() -> Option<String> {
    use crate::share::exec;
    let s = exec("tasklist", [])?;

    for i in s.lines() {
        if i.starts_with("explorer.exe") {
            return Some("Explorer".into());
        }
    }
    s.into()
}
pub fn get_wm_theme() -> Option<String> {
    use crate::share::{exec, get_file_name};
    let s = exec("head", ["-n1", "/proc/registry/HKEY_CURRENT_USER/Software/Microsoft/Windows/CurrentVersion/Themes/CurrentTheme"])  ?;
    let s = get_file_name(&s)?;
    Some(s)
}
