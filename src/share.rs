use std::{
    ffi::OsStr,
    fmt::Debug,
    process::{Command, Stdio},
};
use tracing::instrument;

#[instrument]
pub fn exec<I: Debug, S: Debug>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(cmd)
        // .envs(vars)
        .args(args)
        .stdin(Stdio::null())
        // .stdout(Stdio::null())
        // .stdout(Stdio::null())
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}


#[instrument]
pub async fn exec_async<I: Debug, S: Debug>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = tokio::process::Command::new(cmd)
        // .envs(vars)
        .args(args)
        .stdin(Stdio::null())
        // .stdout(Stdio::null())
        // .stdout(Stdio::null())
        .output().await
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[instrument]
pub fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').next_back()?.split('.').next()?.trim();
    Some(name.into())
}
#[instrument]
pub fn get_pid_name(id: u32) -> Option<String> {
    exec("cat", [format!("/proc/{}/comm", id).as_str()])
}
#[instrument]
pub fn get_ppid(id: u32) -> Option<u32> {
    if let Some(ppid) = exec(
        "grep",
        ["-i", "-F", "PPid:", format!("/proc/{}/status", id).as_str()],
    ) {
        let ppid = ppid.split(':').next_back()?.trim();
        let ppid: u32 = ppid.parse().ok()?;
        return Some(ppid);
    }
    None
}
