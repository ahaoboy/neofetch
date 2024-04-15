use crate::share::exec;

#[derive(Debug, Clone, Copy)]
pub enum Distro {
    Windows,
    Ubuntu,
    Linux,
    Android,
    Unix,
    Mac,
    Unknown,
}

impl ToString for Distro {
    fn to_string(&self) -> String {
        match self {
            Distro::Windows => "Windows".into(),
            Distro::Ubuntu => "Ubuntu".into(),
            Distro::Android => "Android".into(),
            Distro::Linux => "Linux".into(),
            Distro::Unix => "Unix".into(),
            Distro::Mac => "Mac".into(),
            Distro::Unknown => "Unknown".into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OS {
    distro: Distro,
    version: String,
    arch: String,
}
impl ToString for OS {
    fn to_string(&self) -> String {
        [self.distro.to_string().as_str(), &self.version, &self.arch]
            .into_iter()
            .filter(|i| !i.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(windows)]
pub fn get_os() -> Option<OS> {
    let s = exec("wmic", ["os", "get", "Caption"])?;
    let s = s.trim().lines().last()?.trim();
    if s.starts_with("Microsoft Windows 11") {
        return Some(OS {
            distro: Distro::Windows,
            version: "11".into(),
            arch: "x86_64".into(),
        });
    } else if s.starts_with("Microsoft Windows 10") {
        return Some(OS {
            distro: Distro::Windows,
            version: "10".into(),
            arch: "x86_64".into(),
        });
    } else if s.starts_with("Microsoft Windows Server") {
        return Some(OS {
            distro: Distro::Windows,
            version: s.replace("Microsoft Windows Server", "").trim().into(),
            arch: "x86_64".into(),
        });
    }

    None
}

#[cfg(unix)]
pub fn get_os() -> Option<OS> {
    if let (Some(dis), Some(arch)) = (exec("uname", ["-o"]), exec("uname", ["-i"])) {
        match dis.as_str() {
            "Linux" | "GNU/Linux" => {
                if let Some(output) = exec("lsb_release", ["-d"]) {
                    let name = output.split(':').last()?.trim();
                    if name.starts_with("Ubuntu") {
                        return Some(OS {
                            distro: Distro::Ubuntu,
                            arch,
                            version: name[7..].into(),
                        });
                    }
                }

                return Some(OS {
                    distro: Distro::Linux,
                    arch,
                    version: "".into(),
                });
            }
            _ => {}
        }
    }

    // let s = exec("lsb_release", ["-a"])?;
    // let s = s.trim().lines().last()?.trim();
    // if s.starts_with("Microsoft Windows 11") {
    //     return Some(OS {
    //         distro: Distro::Windows,
    //         version: "11".into(),
    //         arch: "x86_64".into(),
    //     });
    // }

    None
}
