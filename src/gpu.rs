use std::fmt::Display;

use human_bytes::human_bytes;

#[derive(Debug, Clone)]
pub struct Gpu {
    pub name: String,
    pub version: String,
    pub ram: u32,
}
impl Display for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} ({}) @ {}",
            self.name,
            human_bytes(self.ram),
            self.version,
        ))
    }
}

#[cfg(windows)]
pub async fn get_gpu() -> Option<Vec<Gpu>> {
    use crate::share::wmi_query;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_VideoController")]
    pub struct VideoController {
        #[serde(rename = "Caption")]
        pub caption: String,
        #[serde(rename = "DriverVersion")]
        pub driver_version: String,
        #[serde(rename = "AdapterRAM")]
        pub adapter_ram: u32,
    }

    let results: Vec<VideoController> = wmi_query().await?;

    Some(
        results
            .iter()
            .map(|i| Gpu {
                name: i.caption.to_owned(),
                version: i.driver_version.to_owned(),
                ram: i.adapter_ram,
            })
            .collect(),
    )
}

#[cfg(unix)]
pub fn get_gpu() -> Option<String> {
    use regex::Regex;

    if let Some(s) = exec("lspci", ["-mm"]) {
        let reg = Regex::new("\"(.*?)\" \"(.*?)\" \"(.*?)\"").unwrap();

        for line in s.lines() {
            let cap = reg.captures(line)?;
            if let (Some(_), Some(a), Some(b)) = (cap.get(1), cap.get(2), cap.get(3)) {
                if ["Display", "3D", "VGA"]
                    .into_iter()
                    .any(|i| b.as_str().contains(i))
                {
                    return Some(a.as_str().to_string() + b.as_str());
                }
            }
        }
    }
    None
}
