use crate::share::exec;
pub fn get_user() -> Option<String> {
    if let Some(s) = exec("id", ["-un"]) {
        return Some(s);
    }

    if let Ok(s) = std::env::var("username") {
        return Some(s);
    }

    if let Ok(s) = std::env::var("HOME") {
        let name = s.replace('\\', "/");
        let name = name.split('/').last()?;
        return Some(name.into());
    }
    Some("".into())
}
