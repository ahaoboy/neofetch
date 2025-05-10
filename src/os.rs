use std::fmt::Display;
use tracing::instrument;


#[derive(Debug, Clone, Copy)]
pub enum Distro {
    Windows,
    Ubuntu,
    Linux,
    Android,
    Unix,
    Darwin,
    Unknown,
}

impl Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Distro::Windows => "Windows",
            Distro::Ubuntu => "Ubuntu",
            Distro::Android => "Android",
            Distro::Linux => "Linux",
            Distro::Unix => "Unix",
            Distro::Darwin => "Darwin",
            Distro::Unknown => "Unknown",
        };
        f.write_str(s)
    }
}
#[derive(Debug, Clone)]
pub struct OS {
    distro: Distro,
    version: String,
    arch: String,
}
impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = [self.distro.to_string().as_str(), &self.version, &self.arch]
            .into_iter()
            .filter(|i| !i.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        f.write_str(&s)
    }
}
#[cfg(windows)]
#[instrument]
pub async fn get_os() -> Option<OS> {
    use crate::share::exec_async;

    let s = exec_async("wmic", ["os", "get", "Caption"])
        .await
        .or(exec_async(
            "powershell",
            [
                "-c",
                "Get-CimInstance Win32_OperatingSystem | Select-Object Caption",
            ],
        )
        .await)?;

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
#[instrument]
pub fn get_os() -> Option<OS> {
    if let (Some(dis), Some(arch)) = (exec("uname", ["-o"]), exec("uname", ["-m"])) {
        match dis.as_str() {
            "Android" => {
                let version = exec("getprop", ["ro.build.version.release"])?;
                return Some(OS {
                    distro: Distro::Android,
                    arch,
                    version,
                });
            }
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
            "Darwin" => {
                return Some(OS {
                    distro: Distro::Darwin,
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
