use crate::share::exec;

#[cfg(windows)]
pub fn get_kernel() -> Option<String> {
    let s = exec("wmic", ["os", "get", "Version"])?;
    s.replace("Version", "").trim().to_string().into()
}
#[cfg(unix)]
pub fn get_kernel() -> Option<String> {
    let s = exec("uname", ["-r"])?;
    s.trim().to_string().into()
}
