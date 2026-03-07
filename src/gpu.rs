use std::fmt::Display;

use human_bytes::human_bytes;

#[derive(Debug, Clone)]
pub struct Gpu {
    pub name: String,
    pub version: String,
    pub ram: u64,
}
impl Display for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];
        v.push(self.name.clone());
        if self.ram > 0 {
            v.push(format!("({})", human_bytes(self.ram as f64)));
        }

        if !self.version.is_empty() {
            v.push(format!("@ ({})", self.version));
        }

        f.write_str(&v.join(" "))
    }
}

#[cfg(windows)]
fn get_gpu_vram_from_registry() -> std::collections::HashMap<String, u64> {
    use winreg::RegKey;
    use winreg::enums::*;

    let mut map = std::collections::HashMap::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_key = match hklm
        .open_subkey(r"SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}")
    {
        Ok(k) => k,
        Err(_) => return map,
    };

    for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
        if let Ok(subkey) = class_key.open_subkey(&subkey_name) {
            let mem = match subkey.get_value::<u64, _>("HardwareInformation.qwMemorySize") {
                Ok(v) => v,
                Err(_) => continue,
            };
            // Index by both DriverDesc and AdapterString for robust matching
            if let Ok(desc) = subkey.get_value::<String, _>("DriverDesc") {
                map.insert(desc, mem);
            }
            if let Ok(adapter_str) =
                subkey.get_value::<String, _>("HardwareInformation.AdapterString")
            {
                map.insert(adapter_str, mem);
            }
        }
    }
    map
}

#[cfg(windows)]
pub async fn get_gpu() -> Option<Vec<Gpu>> {
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_VideoController")]
    pub struct VideoController {
        #[serde(rename = "Caption")]
        pub caption: String,
        #[serde(rename = "DriverVersion")]
        pub driver_version: String,
        #[serde(rename = "AdapterRAM")]
        pub adapter_ram: u64,
    }

    let results: Vec<VideoController> = wmi_query().await.ok()?;
    let vram_map = get_gpu_vram_from_registry();

    Some(
        results
            .iter()
            .map(|i| {
                let ram = vram_map.get(&i.caption).copied().unwrap_or(i.adapter_ram);
                Gpu {
                    name: i.caption.to_owned(),
                    version: i.driver_version.to_owned(),
                    ram,
                }
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
        } else if let Some(vendor_id) = &current_vendor
            && let Some((id, name)) = line.trim().split_once(' ')
        {
            let id = id.trim().to_lowercase();
            let name = name.trim().to_string();
            devices.insert((vendor_id.clone(), id), name);
        }
    }

    (vendors, devices)
}

#[cfg(unix)]
pub async fn get_gpu() -> Option<Vec<Gpu>> {
    let path = std::path::Path::new("/sys/bus/pci/devices");
    let (vendor_names, device_names) = load_pci_ids();

    let mut v = vec![];
    let mut dir = tokio::fs::read_dir(path).await.ok()?;
    while let Some(entry) = dir.next_entry().await.ok()? {
        let device_path = entry.path();

        let vendor = tokio::fs::read_to_string(device_path.join("vendor"))
            .await
            .unwrap_or_default();
        let device = tokio::fs::read_to_string(device_path.join("device"))
            .await
            .unwrap_or_default();

        let vendor_id = vendor.trim_start_matches("0x").trim().to_lowercase();
        let device_id = device.trim_start_matches("0x").trim().to_lowercase();

        if let (Some(vendor_name), Some(device_name)) = (
            vendor_names.get(&vendor_id),
            device_names.get(&(vendor_id.clone(), device_id.clone())),
        ) && ["Display", "3D", "VGA"]
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
    Some(v)
}
