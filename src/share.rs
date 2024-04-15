use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

pub fn exec<I, S>(cmd: S, args: I) -> Option<String>
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

pub fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').last()?.split('.').next()?.trim();
    Some(name.into())
}

pub fn get_pid_name(id: u32) -> Option<String> {
    exec("cat", [format!("/proc/{}/comm", id).as_str()])
}

pub fn get_ppid(id: u32) -> Option<u32> {
    if let Some(ppid) = exec(
        "grep",
        ["-i", "-F", "PPid:", format!("/proc/{}/status", id).as_str()],
    ) {
        let ppid = ppid.split(':').last()?.trim();
        let ppid: u32 = ppid.parse().ok()?;
        return Some(ppid);
    }
    None
}