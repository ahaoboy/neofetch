use crate::share::exec;

#[cfg(windows)]
pub fn get_resolution() -> Option<String> {
    let s = exec(
        "wmic",
        [
            "path",
            "Win32_VideoController",
            "get",
            "CurrentHorizontalResolution",
        ],
    ).or(exec(
      "powershell",
      [
          "-c",
          "Get-CimInstance Win32_VideoController | Select-Object CurrentHorizontalResolution",
      ],
  ))?;

    let w = s.trim().lines().last()?.trim();
    let s = exec(
        "wmic",
        [
            "path",
            "Win32_VideoController",
            "get",
            "CurrentVerticalResolution",
        ],
    ).or(exec(
      "powershell",
      [
          "-c",
          "Get-CimInstance Win32_VideoController | Select-Object CurrentVerticalResolution",
      ],
  ))?;
    let h = s.trim().lines().last()?.trim();
    Some(format!("{w}x{h}"))
}

#[cfg(unix)]
pub fn get_resolution() -> Option<String> {
    use regex::Regex;
    if let Some(s) = exec("xrandr", ["--nograb", "--current"]) {
        // Screen 0: minimum 16 x 16, current 1618 x 1172, maximum 32767 x 32767
        let re = Regex::new(r"current (.*?) x (.*?),").unwrap();
        if let Some(caps) = re.captures(&s) {
            if let (Some(w), Some(h)) = (caps.get(1), caps.get(2)) {
                return Some(format!("{} x {}", w.as_str(), h.as_str()));
            }
        }
    }
    if let Some(s) = exec("bash", ["-c", "cat /sys/class/drm/*/modes"]) {
        let mut s = s.lines().next()?.split('x');
        if let (Some(w), Some(h)) = (s.next(), s.next()) {
            return Some(format!("{} x {}", w, h));
        }
    }
    None
}
