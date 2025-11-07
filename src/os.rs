//! Operating system information collector
//!
//! Collects OS name, version, and distribution information across platforms.

use crate::error::{NeofetchError, Result};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Distro {
    Aix,
    AlmaLinux,
    AlpineSmall,
    Alpine,
    Alter,
    Amazon,
    Anarchy,
    AndroidSmall,
    Android,
    InstantOs,
    Antergos,
    AntiX,
    AoscOs,
    Apricity,
    Archcraft,
    ArcolinuxSmall,
    ArcoLinux,
    ArchSmall,
    ArchOld,
    ArchBox,
    ArcHlabs,
    ArchStrike,
    XFerience,
    ArchMerge,
    Arch,
    ArtixSmall,
    Artix,
    Arya,
    Bedrock,
    Bitrig,
    BlackArch,
    Blag,
    BlankOn,
    BlueLight,
    Bonsai,
    BunsenLabs,
    Calculate,
    Carbs,
    CblMariner,
    CelOs,
    CentosSmall,
    CentOs,
    Chakra,
    ChaletOs,
    Chapeau,
    Chrom,
    CleanjaroSmall,
    Cleanjaro,
    ClearOs,
    ClearLinuxOs,
    Clover,
    Condres,
    ContainerLinuxByCoreOs,
    CruxSmall,
    Crux,
    CrystalLinux,
    Cucumber,
    Dahlia,
    DebianSmall,
    Debian,
    Deepin,
    DesaOs,
    Devuan,
    DracOs,
    Itc,
    DragonflyOld,
    DragonflySmall,
    DragonFly,
    Drauger,
    ElementarySmall,
    Elementary,
    EndeavourOs,
    Endless,
    EuroLinux,
    Exherbo,
    FedoraSmall,
    FedoraOld,
    Fedora,
    Feren,
    FreebsdSmall,
    FreeMiNt,
    Frugalware,
    Funtoo,
    GalliumOs,
    Garuda,
    GentooSmall,
    Gentoo,
    Pentoo,
    Glaucus,
    GNewSense,
    Gnome,
    Gnu,
    GoboLinux,
    Grombyang,
    GuixSmall,
    Guix,
    HaikuSmall,
    Haiku,
    Huayra,
    HydroOs,
    HyperbolaSmall,
    Hyperbola,
    Iglunix,
    Januslinux,
    Kaisen,
    Kali,
    KaOs,
    Kde,
    Kibojoe,
    Kogaion,
    Korora,
    KsLinux,
    Kubuntu,
    Lede,
    LaxerOs,
    LibreElec,
    Linux,
    LinuxliteSmall,
    LinuxLite,
    Lmde,
    Lubuntu,
    Lunar,
    Mac,
    MageiaSmall,
    Mageia,
    MagpieOs,
    Mandriva,
    ManjaroSmall,
    Manjaro,
    Maui,
    Mer,
    Minix,
    LinuxmintSmall,
    LinuxMintOld,
    LinuxMint,
    LiveRaizo,
    MxSmall,
    Mx,
    Namib,
    Neptune,
    NetbsdSmall,
    NetBsd,
    Netrunner,
    Nitrux,
    NixosSmall,
    NixosOld,
    NixOs,
    Nurunner,
    NuTyX,
    ObRevenge,
    OpenbsdSmall,
    OpenBsd,
    OpenEuler,
    OpenIndiana,
    Openmamba,
    OpenMandriva,
    OpenStage,
    OpenWrt,
    OpenSourceMediaCenter,
    Oracle,
    OsElbrus,
    PacBsd,
    ParabolaSmall,
    Parabola,
    Pardus,
    Parrot,
    Parsix,
    Pcbsd,
    PcLinuxOs,
    Pengwin,
    Peppermint,
    PoposSmall,
    PopOs,
    Porteus,
    PostmarketosSmall,
    PostMarketOs,
    PuffOs,
    Puppy,
    PureosSmall,
    PureOs,
    Qubes,
    Qubyt,
    Quibian,
    Radix,
    RaspbianSmall,
    Raspbian,
    RebornOs,
    RedStar,
    Redcore,
    RedhatOld,
    Redhat,
    RefractedDevuan,
    Regata,
    Regolith,
    RockySmall,
    Rosa,
    Sabotage,
    Sabayon,
    Sailfish,
    SalentOs,
    Scientific,
    Septor,
    Serene,
    SharkLinux,
    SlackwareSmall,
    Slackware,
    SliTaz,
    SmartOs,
    Solus,
    SourceMage,
    Sparky,
    Star,
    SteamOs,
    SunosSmall,
    OpenSuseLeap,
    T2,
    OpenSuseTumbleweed,
    OpensuseSmall,
    OpenSuse,
    SwagArch,
    Tails,
    Trisquel,
    UbuntuCinnamon,
    UbuntuBudgie,
    UbuntuGnome,
    UbuntuMate,
    UbuntuOld,
    UbuntuStudio,
    UbuntuSmall,
    Ubuntu,
    Univention,
    Venom,
    VoidSmall,
    LangitKetujuh,
    Semc,
    Obarun,
    Windows11,
    Windows10,
    Windows,
    Xubuntu,
    Zorin,
    Darwin,
    ProfelisSambaBox,
    Unknown,
}
impl Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Distro::Aix => "Aix",
            Distro::AlmaLinux => "AlmaLinux",
            Distro::AlpineSmall => "AlpineSmall",
            Distro::Alpine => "Alpine",
            Distro::Alter => "Alter",
            Distro::Amazon => "Amazon",
            Distro::Anarchy => "Anarchy",
            Distro::AndroidSmall => "AndroidSmall",
            Distro::Android => "Android",
            Distro::InstantOs => "InstantOs",
            Distro::Antergos => "Antergos",
            Distro::AntiX => "AntiX",
            Distro::AoscOs => "AoscOs",
            Distro::Apricity => "Apricity",
            Distro::Archcraft => "Archcraft",
            Distro::ArcolinuxSmall => "ArcolinuxSmall",
            Distro::ArcoLinux => "ArcoLinux",
            Distro::ArchSmall => "ArchSmall",
            Distro::ArchOld => "ArchOld",
            Distro::ArchBox => "ArchBox",
            Distro::ArcHlabs => "ArcHlabs",
            Distro::ArchStrike => "ArchStrike",
            Distro::XFerience => "XFerience",
            Distro::ArchMerge => "ArchMerge",
            Distro::Arch => "Arch",
            Distro::ArtixSmall => "ArtixSmall",
            Distro::Artix => "Artix",
            Distro::Arya => "Arya",
            Distro::Bedrock => "Bedrock",
            Distro::Bitrig => "Bitrig",
            Distro::BlackArch => "BlackArch",
            Distro::Blag => "Blag",
            Distro::BlankOn => "BlankOn",
            Distro::BlueLight => "BlueLight",
            Distro::Bonsai => "Bonsai",
            Distro::BunsenLabs => "BunsenLabs",
            Distro::Calculate => "Calculate",
            Distro::Carbs => "Carbs",
            Distro::CblMariner => "CblMariner",
            Distro::CelOs => "CelOs",
            Distro::CentosSmall => "CentosSmall",
            Distro::CentOs => "CentOs",
            Distro::Chakra => "Chakra",
            Distro::ChaletOs => "ChaletOs",
            Distro::Chapeau => "Chapeau",
            Distro::Chrom => "Chrom",
            Distro::CleanjaroSmall => "CleanjaroSmall",
            Distro::Cleanjaro => "Cleanjaro",
            Distro::ClearOs => "ClearOs",
            Distro::ClearLinuxOs => "ClearLinuxOs",
            Distro::Clover => "Clover",
            Distro::Condres => "Condres",
            Distro::ContainerLinuxByCoreOs => "ContainerLinuxByCoreOs",
            Distro::CruxSmall => "CruxSmall",
            Distro::Crux => "Crux",
            Distro::CrystalLinux => "CrystalLinux",
            Distro::Cucumber => "Cucumber",
            Distro::Dahlia => "Dahlia",
            Distro::DebianSmall => "DebianSmall",
            Distro::Debian => "Debian",
            Distro::Deepin => "Deepin",
            Distro::DesaOs => "DesaOs",
            Distro::Devuan => "Devuan",
            Distro::DracOs => "DracOs",
            Distro::Itc => "Itc",
            Distro::DragonflyOld => "DragonflyOld",
            Distro::DragonflySmall => "DragonflySmall",
            Distro::DragonFly => "DragonFly",
            Distro::Drauger => "Drauger",
            Distro::ElementarySmall => "ElementarySmall",
            Distro::Elementary => "Elementary",
            Distro::EndeavourOs => "EndeavourOs",
            Distro::Endless => "Endless",
            Distro::EuroLinux => "EuroLinux",
            Distro::Exherbo => "Exherbo",
            Distro::FedoraSmall => "FedoraSmall",
            Distro::FedoraOld => "FedoraOld",
            Distro::Fedora => "Fedora",
            Distro::Feren => "Feren",
            Distro::FreebsdSmall => "FreebsdSmall",
            Distro::FreeMiNt => "FreeMiNt",
            Distro::Frugalware => "Frugalware",
            Distro::Funtoo => "Funtoo",
            Distro::GalliumOs => "GalliumOs",
            Distro::Garuda => "Garuda",
            Distro::GentooSmall => "GentooSmall",
            Distro::Gentoo => "Gentoo",
            Distro::Pentoo => "Pentoo",
            Distro::Glaucus => "Glaucus",
            Distro::GNewSense => "GNewSense",
            Distro::Gnome => "Gnome",
            Distro::Gnu => "Gnu",
            Distro::GoboLinux => "GoboLinux",
            Distro::Grombyang => "Grombyang",
            Distro::GuixSmall => "GuixSmall",
            Distro::Guix => "Guix",
            Distro::HaikuSmall => "HaikuSmall",
            Distro::Haiku => "Haiku",
            Distro::Huayra => "Huayra",
            Distro::HydroOs => "HydroOs",
            Distro::HyperbolaSmall => "HyperbolaSmall",
            Distro::Hyperbola => "Hyperbola",
            Distro::Iglunix => "Iglunix",
            Distro::Januslinux => "Januslinux",
            Distro::Kaisen => "Kaisen",
            Distro::Kali => "Kali",
            Distro::KaOs => "KaOs",
            Distro::Kde => "Kde",
            Distro::Kibojoe => "Kibojoe",
            Distro::Kogaion => "Kogaion",
            Distro::Korora => "Korora",
            Distro::KsLinux => "KsLinux",
            Distro::Kubuntu => "Kubuntu",
            Distro::Lede => "Lede",
            Distro::LaxerOs => "LaxerOs",
            Distro::LibreElec => "LibreElec",
            Distro::Linux => "Linux",
            Distro::LinuxliteSmall => "LinuxliteSmall",
            Distro::LinuxLite => "LinuxLite",
            Distro::Lmde => "Lmde",
            Distro::Lubuntu => "Lubuntu",
            Distro::Lunar => "Lunar",
            Distro::Mac => "Mac",
            Distro::MageiaSmall => "MageiaSmall",
            Distro::Mageia => "Mageia",
            Distro::MagpieOs => "MagpieOs",
            Distro::Mandriva => "Mandriva",
            Distro::ManjaroSmall => "ManjaroSmall",
            Distro::Manjaro => "Manjaro",
            Distro::Maui => "Maui",
            Distro::Mer => "Mer",
            Distro::Minix => "Minix",
            Distro::LinuxmintSmall => "LinuxmintSmall",
            Distro::LinuxMintOld => "LinuxMintOld",
            Distro::LinuxMint => "LinuxMint",
            Distro::LiveRaizo => "LiveRaizo",
            Distro::MxSmall => "MxSmall",
            Distro::Mx => "Mx",
            Distro::Namib => "Namib",
            Distro::Neptune => "Neptune",
            Distro::NetbsdSmall => "NetbsdSmall",
            Distro::NetBsd => "NetBsd",
            Distro::Netrunner => "Netrunner",
            Distro::Nitrux => "Nitrux",
            Distro::NixosSmall => "NixosSmall",
            Distro::NixosOld => "NixosOld",
            Distro::NixOs => "NixOs",
            Distro::Nurunner => "Nurunner",
            Distro::NuTyX => "NuTyX",
            Distro::ObRevenge => "ObRevenge",
            Distro::OpenbsdSmall => "OpenbsdSmall",
            Distro::OpenBsd => "OpenBsd",
            Distro::OpenEuler => "OpenEuler",
            Distro::OpenIndiana => "OpenIndiana",
            Distro::Openmamba => "Openmamba",
            Distro::OpenMandriva => "OpenMandriva",
            Distro::OpenStage => "OpenStage",
            Distro::OpenWrt => "OpenWrt",
            Distro::OpenSourceMediaCenter => "OpenSourceMediaCenter",
            Distro::Oracle => "Oracle",
            Distro::OsElbrus => "OsElbrus",
            Distro::PacBsd => "PacBsd",
            Distro::ParabolaSmall => "ParabolaSmall",
            Distro::Parabola => "Parabola",
            Distro::Pardus => "Pardus",
            Distro::Parrot => "Parrot",
            Distro::Parsix => "Parsix",
            Distro::Pcbsd => "Pcbsd",
            Distro::PcLinuxOs => "PcLinuxOs",
            Distro::Pengwin => "Pengwin",
            Distro::Peppermint => "Peppermint",
            Distro::PoposSmall => "PoposSmall",
            Distro::PopOs => "PopOs",
            Distro::Porteus => "Porteus",
            Distro::PostmarketosSmall => "PostmarketosSmall",
            Distro::PostMarketOs => "PostMarketOs",
            Distro::PuffOs => "PuffOs",
            Distro::Puppy => "Puppy",
            Distro::PureosSmall => "PureosSmall",
            Distro::PureOs => "PureOs",
            Distro::Qubes => "Qubes",
            Distro::Qubyt => "Qubyt",
            Distro::Quibian => "Quibian",
            Distro::Radix => "Radix",
            Distro::RaspbianSmall => "RaspbianSmall",
            Distro::Raspbian => "Raspbian",
            Distro::RebornOs => "RebornOs",
            Distro::RedStar => "RedStar",
            Distro::Redcore => "Redcore",
            Distro::RedhatOld => "RedhatOld",
            Distro::Redhat => "Redhat",
            Distro::RefractedDevuan => "RefractedDevuan",
            Distro::Regata => "Regata",
            Distro::Regolith => "Regolith",
            Distro::RockySmall => "RockySmall",
            Distro::Rosa => "Rosa",
            Distro::Sabotage => "Sabotage",
            Distro::Sabayon => "Sabayon",
            Distro::Sailfish => "Sailfish",
            Distro::SalentOs => "SalentOs",
            Distro::Scientific => "Scientific",
            Distro::Septor => "Septor",
            Distro::Serene => "Serene",
            Distro::SharkLinux => "SharkLinux",
            Distro::SlackwareSmall => "SlackwareSmall",
            Distro::Slackware => "Slackware",
            Distro::SliTaz => "SliTaz",
            Distro::SmartOs => "SmartOs",
            Distro::Solus => "Solus",
            Distro::SourceMage => "SourceMage",
            Distro::Sparky => "Sparky",
            Distro::Star => "Star",
            Distro::SteamOs => "SteamOs",
            Distro::SunosSmall => "SunosSmall",
            Distro::OpenSuseLeap => "OpenSuseLeap",
            Distro::T2 => "T2",
            Distro::OpenSuseTumbleweed => "OpenSuseTumbleweed",
            Distro::OpensuseSmall => "OpensuseSmall",
            Distro::OpenSuse => "OpenSuse",
            Distro::SwagArch => "SwagArch",
            Distro::Tails => "Tails",
            Distro::Trisquel => "Trisquel",
            Distro::UbuntuCinnamon => "UbuntuCinnamon",
            Distro::UbuntuBudgie => "UbuntuBudgie",
            Distro::UbuntuGnome => "UbuntuGnome",
            Distro::UbuntuMate => "UbuntuMate",
            Distro::UbuntuOld => "UbuntuOld",
            Distro::UbuntuStudio => "UbuntuStudio",
            Distro::UbuntuSmall => "UbuntuSmall",
            Distro::Ubuntu => "Ubuntu",
            Distro::Univention => "Univention",
            Distro::Venom => "Venom",
            Distro::VoidSmall => "VoidSmall",
            Distro::LangitKetujuh => "LangitKetujuh",
            Distro::Semc => "Semc",
            Distro::Obarun => "Obarun",
            Distro::Windows11 => "Windows11",
            Distro::Windows10 => "Windows10",
            Distro::Windows => "Windows",
            Distro::Xubuntu => "Xubuntu",
            Distro::Zorin => "Zorin",
            Distro::Darwin => "Darwin",
            Distro::ProfelisSambaBox => "ProfelisSambaBox",
            Distro::Unknown => "Unknown",
        };
        f.write_str(s)
    }
}
#[derive(Debug, Clone)]
pub struct OS {
    pub distro: Distro,
    pub name: String,
    pub arch: String,
}
impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{} {}", self.name, self.arch);
        f.write_str(&s)
    }
}
/// Get OS information on Windows
#[cfg(windows)]
pub async fn get_os() -> Result<OS> {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "Caption")]
        caption: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_Processor")]
    struct Processor {
        #[serde(rename = "Architecture")]
        architecture: u32,
    }

    // Query OS name
    let com = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con = wmi::WMIConnection::new(com)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;

    let os_results: Vec<OperatingSystem> = wmi_con
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("OS query failed: {}", e)))?;

    let name = os_results
        .first()
        .ok_or_else(|| NeofetchError::data_unavailable("No OS information found"))?
        .caption
        .trim()
        .replace("Microsoft ", "");

    // Query processor architecture
    let com2 = wmi::COMLibrary::new()
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to initialize COM: {}", e)))?;
    let wmi_con2 = wmi::WMIConnection::new(com2)
        .map_err(|e| NeofetchError::wmi_error(format!("Failed to connect to WMI: {}", e)))?;

    let proc_results: Vec<Processor> = wmi_con2
        .async_query()
        .await
        .map_err(|e| NeofetchError::wmi_error(format!("Processor query failed: {}", e)))?;

    let arch = proc_results
        .first()
        .map(|p| match p.architecture {
            0 => "x86",
            9 => "x86_64",
            12 => "ARM",
            5 => "ARM64",
            _ => "Unknown",
        })
        .unwrap_or("Unknown")
        .to_string();

    // Determine distro variant
    let distro = if name.starts_with("Windows 11") {
        Distro::Windows11
    } else if name.starts_with("Windows 10") {
        Distro::Windows10
    } else {
        Distro::Windows
    };

    Ok(OS { distro, name, arch })
}

/// Get OS information on Unix-like systems
#[cfg(unix)]
pub async fn get_os() -> Result<OS> {
    use crate::utils::read_file_to_string;
    use std::ffi::CStr;

    // Get architecture using uname
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::uname(&mut uts) };
    if result != 0 {
        return Err(NeofetchError::system_call("uname failed"));
    }

    let arch = unsafe { CStr::from_ptr(uts.machine.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    let sysname = std::env::consts::OS.to_string();

    match sysname.to_lowercase().as_str() {
        #[cfg(target_os = "android")]
        "android" => {
            let version = crate::share::get_property("ro.build.version.release");
            let name = if let Some(v) = version {
                format!("Android {}", v)
            } else {
                "Android".to_string()
            };

            Ok(OS {
                distro: Distro::Android,
                arch,
                name,
            })
        }
        "linux" | "gnu/linux" => {
            // Try to read /etc/os-release for distribution info
            if let Ok(content) = read_file_to_string("/etc/os-release").await {
                for line in content.lines() {
                    if let Some(pretty_name) = line.strip_prefix("PRETTY_NAME=") {
                        let name = pretty_name.trim_matches('"').to_string();

                        // Detect specific distributions
                        let distro = if name.contains("Ubuntu") {
                            Distro::Ubuntu
                        } else if name.contains("Debian") {
                            Distro::Debian
                        } else if name.contains("Fedora") {
                            Distro::Fedora
                        } else if name.contains("Arch") {
                            Distro::Arch
                        } else if name.contains("Manjaro") {
                            Distro::Manjaro
                        } else {
                            Distro::Linux
                        };

                        return Ok(OS { distro, arch, name });
                    }
                }
            }

            // Fallback to generic Linux
            Ok(OS {
                distro: Distro::Linux,
                arch,
                name: "Linux".to_string(),
            })
        }
        "darwin" | "macos" | "ios" => Ok(OS {
            distro: Distro::Darwin,
            arch,
            name: "Darwin".to_string(),
        }),
        _ => Err(NeofetchError::UnsupportedPlatform),
    }
}
