#[derive(Debug, Clone)]
pub struct Display {
    pub name: Option<String>,
    pub refresh_rate: Option<u32>,
    pub external: Option<bool>,
    pub resolution: Option<(u32, u32)>,
    pub scale_resolution: Option<(u32, u32)>,
    pub rotation: Option<u32>,
    pub primary: Option<bool>,
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];
        if let Some((w, h)) = self.resolution {
            v.push(format!("{}x{}", w, h));
        }

        if let Some(refresh_rate) = self.refresh_rate {
            v.push(format!("@ {}Hz", refresh_rate));
        }

        if let Some((w, h)) = self.scale_resolution {
            v.push(format!("(as {}x{})", w, h));
        }

        f.write_str(&v.join(" "))
    }
}

use crate::share::exec;

#[cfg(windows)]
pub fn get_display() -> Option<Vec<Display>> {
    let pnp_id_str = exec("wmic", ["desktopmonitor", "get", "PNPDeviceID"])?;
    let mut v = vec![];

    let display_str = exec(
        "wmic",
        [
            "path",
            "Win32_VideoController",
            "get",
            "CurrentHorizontalResolution,CurrentVerticalResolution,CurrentRefreshRate",
            "/format:csv",
        ],
    )?;

    let name_list = pnp_id_str.lines().skip(1).filter_map(|line| {
        if let Some(name) = exec(
            "powershell",
            [
                "-c",
                &format!(
                    "Get-PnpDevice -InstanceId  '{}' |  Select-Object FriendlyName",
                    line.trim()
                ),
            ],
        ) {
            let mut name = name.lines().last().unwrap().trim().to_string();
            if name.starts_with("Generic Monitor (") {
                name = name[17..name.len() - 1].trim().to_string()
            }
            return Some(name);
        }
        None
    });

    let info_list = display_str.lines().skip(1).flat_map(|s| {
        let v = s.split(',').collect::<Vec<_>>();
        let w = v[1].parse::<u32>().ok()?;
        let h = v[3].parse::<u32>().ok()?;
        let refresh_rate = v[2].parse::<u32>().ok()?;
        Some((w, h, refresh_rate))
    });

    for (name, (w, h, refresh_rate)) in name_list.zip(info_list) {
        v.push(Display {
            name: Some(name),
            refresh_rate: Some(refresh_rate),
            external: None,
            resolution: Some((w, h)),
            scale_resolution: None,
            rotation: None,
            primary: None,
        })
    }

    Some(v)
}

#[cfg(unix)]
pub fn get_display() -> Option<Vec<Display>> {
    None
}
