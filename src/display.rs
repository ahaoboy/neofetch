#[derive(Debug, Clone)]
pub struct Display {
    pub name: Option<String>,
    pub friendly_name: Option<String>,
    pub refresh_rate: Option<f32>,
    pub external: Option<bool>,
    pub resolution: Option<(u32, u32)>,
    pub scale_resolution: Option<(u32, u32)>,
    pub rotation: Option<f32>,
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

        if let Some(true) = self.external {
            v.push("[External]".to_string());
        }

        if let Some(true) = self.primary {
            v.push("*".to_string());
        }
        f.write_str(&v.join(" "))
    }
}
use tracing::instrument;

#[cfg(not(target_os = "android"))]
#[instrument]
pub async fn get_display() -> Option<Vec<Display>> {
    use display_info::DisplayInfo;

    let display_infos = DisplayInfo::all().ok()?;

    let v: Vec<_> = display_infos
        .iter()
        .map(|i| Display {
            name: Some(i.name.clone()),
            friendly_name: if i.friendly_name.starts_with("Unknown Display") {
                None
            } else {
                Some(i.friendly_name.clone())
            },
            refresh_rate: Some(i.frequency),
            external: None,
            resolution: Some((i.width, i.height)),
            scale_resolution: Some((
                (i.scale_factor * i.width as f32) as u32,
                (i.scale_factor * i.height as f32) as u32,
            )),
            rotation: Some(i.rotation),
            primary: Some(i.is_primary),
        })
        .collect();
    Some(v)
}

#[cfg(target_os = "android")]
#[instrument]
pub fn get_display() -> Option<Vec<Display>> {
    None
}
