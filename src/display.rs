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

        if let Some(true) = self.external {
            v.push("[External]".to_string());
        }

        if let Some(true) = self.primary {
            v.push("*".to_string());
        }
        f.write_str(&v.join(" "))
    }
}

#[cfg(windows)]
pub fn get_display() -> Option<Vec<Display>> {
    use glfw::ffi::GLFWmonitor;

    fn get_ptr(m: &glfw::Monitor) -> usize {
        pub struct A {
            ptr: *mut GLFWmonitor,
        }
        let a = m as *const _ as *const A;
        unsafe { (*a).ptr as usize }
    }

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let p = glfw
        .with_primary_monitor(|_, m: Option<&mut glfw::Monitor>| m.map(|i| get_ptr(i)))
        .unwrap_or_default();

    let v = glfw.with_connected_monitors(|_, monitors| {
        monitors
            .iter()
            .map(|monitor| {
                let mode = monitor.get_video_mode().unwrap();
                let refresh_rate = mode.refresh_rate;
                let resolution = (mode.width, mode.height);
                let scale = monitor.get_content_scale();
                let scale_resolution = (
                    (resolution.0 as f32 / scale.0) as u32,
                    (resolution.1 as f32 / scale.1) as u32,
                );
                let primary = get_ptr(monitor) == p;

                Display {
                    name: monitor.get_name(),
                    refresh_rate: Some(refresh_rate),
                    external: None,
                    resolution: Some(resolution),
                    scale_resolution: Some(scale_resolution),
                    rotation: None,
                    primary: Some(primary),
                }
            })
            .collect::<Vec<_>>()
    });
    Some(v)
}

#[cfg(unix)]
pub fn get_display() -> Option<Vec<Display>> {
    None
}
