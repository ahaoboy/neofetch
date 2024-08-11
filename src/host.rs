use crate::share::exec;
#[cfg(windows)]
pub fn get_host() -> Option<String> {
    let s = exec("wmic", ["computersystem", "get", "manufacturer"]).or(exec(
      "powershell",
      [
          "-c",
          "Get-CimInstance Win32_computersystem | Select-Object manufacturer",
      ],
  ))?;
    s.lines().last()?.trim().to_string().into()
}

#[cfg(windows)]
pub fn get_rom() -> Option<String> {
    None
}

#[cfg(windows)]
pub fn get_baseband() -> Option<String> {
    None
}

#[cfg(unix)]
pub fn get_rom() -> Option<String> {
    exec("getprop", ["ro.build.display.id"])
}

#[cfg(unix)]
pub fn get_baseband() -> Option<String> {
    exec("getprop", ["ro.baseband"])
}

#[cfg(unix)]
pub fn get_host() -> Option<String> {
    if let (Some(name), Some(version)) = (
        exec("cat", ["/sys/devices/virtual/dmi/id/board_name"]),
        exec("cat", ["/sys/devices/virtual/dmi/id/product_version"]),
    ) {
        if !name.is_empty() && !version.is_empty() {
            return format!("{name} {version}").into();
        }
    }

    if let (Some(name), Some(version), Some(device)) = (
        exec("getprop", ["ro.product.brand"]),
        exec("getprop", ["ro.product.model"]),
        exec("getprop", ["ro.product.device"]),
    ) {
        if !name.is_empty() && !version.is_empty() && !device.is_empty() {
            return format!("{name} {version} ({device})").into();
        }
    }
    None
}
