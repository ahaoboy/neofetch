use std::{
    ffi::OsStr,
    fmt::Debug,
    process::{Command, Stdio},
};

pub fn exec<I, S>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    let output = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub async fn exec_async<S, I>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    let output = tokio::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .await
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').next_back()?.split('.').next()?.trim();
    Some(name.into())
}
pub fn get_pid_name(id: u32) -> Option<String> {
    exec("cat", [format!("/proc/{id}/comm").as_str()])
}
pub fn get_ppid(id: u32) -> Option<u32> {
    if let Some(ppid) = exec(
        "grep",
        ["-i", "-F", "PPid:", format!("/proc/{id}/status").as_str()],
    ) {
        let ppid = ppid.split(':').next_back()?.trim();
        let ppid: u32 = ppid.parse().ok()?;
        return Some(ppid);
    }
    None
}

#[cfg(windows)]
pub async fn wmi_query<T: serde::de::DeserializeOwned>() -> Option<Vec<T>> {
    use wmi::{COMLibrary, WMIConnection};
    let com = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com).ok()?;
    let results: Vec<T> = wmi_con.async_query().await.ok()?;
    Some(results)
}
