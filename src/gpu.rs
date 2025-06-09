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
        let mut v = vec![];
        v.push(self.name.clone());
        if self.ram > 0 {
            v.push(format!("({})", human_bytes(self.ram)));
        }

        if !self.version.is_empty() {
            v.push(format!("@ ({})", self.version));
        }

        f.write_str(&v.join(" "))
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
fn load_pci_ids() -> (
    std::collections::HashMap<String, String>,
    std::collections::HashMap<(String, String), String>,
) {
    let content = std::fs::read_to_string("/usr/share/misc/pci.ids").unwrap_or_default();
    let mut vendors = std::collections::HashMap::new();
    let mut devices = std::collections::HashMap::new();

    let mut current_vendor: Option<String> = None;

    for line in content.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if !line.starts_with('\t') {
            if let Some((id, name)) = line.split_once(' ') {
                let id = id.trim().to_lowercase();
                let name = name.trim().to_string();
                current_vendor = Some(id.clone());
                vendors.insert(id, name);
            }
        } else if let Some(vendor_id) = &current_vendor {
            if let Some((id, name)) = line.trim().split_once(' ') {
                let id = id.trim().to_lowercase();
                let name = name.trim().to_string();
                devices.insert((vendor_id.clone(), id), name);
            }
        }
    }

    (vendors, devices)
}

#[cfg(unix)]
pub async fn get_gpu() -> Option<Vec<Gpu>> {
    let path = std::path::Path::new("/sys/bus/pci/devices");
    let (vendor_names, device_names) = load_pci_ids();

    let mut v = vec![];
    for entry in std::fs::read_dir(path).ok()? {
        let entry = entry.unwrap();
        let device_path = entry.path();

        let vendor = std::fs::read_to_string(device_path.join("vendor")).unwrap_or_default();
        let device = std::fs::read_to_string(device_path.join("device")).unwrap_or_default();

        let vendor_id = vendor.trim_start_matches("0x").trim().to_lowercase();
        let device_id = device.trim_start_matches("0x").trim().to_lowercase();

        if let (Some(vendor_name), Some(device_name)) = (
            vendor_names.get(&vendor_id),
            device_names.get(&(vendor_id.clone(), device_id.clone())),
        ) {
            if ["Display", "3D", "VGA"]
                .into_iter()
                .any(|i| device_name.as_str().contains(i))
            {
                v.push(Gpu {
                    name: vendor_name.to_owned(),
                    version: device_name.to_owned(),
                    ram: 0,
                });
            }
        }
    }
    Some(v)
}
