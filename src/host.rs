use crate::share::exec;
#[cfg(windows)]
pub fn get_host() -> Option<String> {
    let s = exec("wmic", ["computersystem", "get", "manufacturer"])?;
    s.lines().last()?.trim().to_string().into()
}
#[cfg(unix)]
pub fn get_host() -> Option<String> {
    if let (Some(name), Some(version)) = (
        exec("cat", ["/sys/devices/virtual/dmi/id/board_name"]),
        exec("cat", ["/sys/devices/virtual/dmi/id/product_version"]),
    ) {
        return format!("{name} {version}").into();
    }
    None
}
