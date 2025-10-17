pub mod battery;
pub mod color;
pub mod cpu;
pub mod de;
pub mod disk;
pub mod display;
pub mod gpu;
pub mod host;
pub mod hostname;
pub mod icon;
pub mod ip;
pub mod kernel;
pub mod locale;
pub mod mappings;
pub mod memory;
pub mod os;
pub mod packages;
pub mod share;
pub mod shell;
pub mod terminal;
pub mod uptime;
pub mod user;
pub mod wm;
use cpu::Cpu;
use disk::Disk;
use display::{Display, get_display};
use gpu::Gpu;
use hostname::get_hostname;
use os::OS;
use packages::Packages;
use uptime::Time;
use which_shell::ShellVersion;

use crate::battery::get_battery;
use crate::color::{
    BLACK_BG, BLUE_BG, BOLD, BRIGHT_BLACK_BG, BRIGHT_BLUE_BG, BRIGHT_CYAN_BG, BRIGHT_GREEN_BG,
    BRIGHT_MAGENTA_BG, BRIGHT_RED_BG, BRIGHT_WHITE_BG, BRIGHT_YELLOW_BG, CYAN_BG, GREEN, GREEN_BG,
    MAGENTA_BG, RED, RED_BG, RESET, WHITE_BG, YELLOW_BG, cursor_down, cursor_forward, cursor_up,
};
use crate::cpu::get_cpu;
use crate::de::get_de;
use crate::disk::get_disk;
use crate::host::get_host;
use crate::host::{get_baseband, get_rom};
use crate::kernel::get_kernel;
use crate::locale::get_locale;
use crate::memory::get_memory;
use crate::packages::get_packages;
use crate::shell::which_shell;
use crate::terminal::get_terminal;
use crate::uptime::get_uptime;
use crate::user::get_user;
use crate::wm::{get_wm, get_wm_theme};
use crate::{gpu::get_gpu, os::get_os};

pub fn join(left: String, right: String) -> String {
    let mut s = String::new();
    let left_h = left.lines().count();
    let right_h = right.lines().count();
    let max_h = left_h.max(right_h);
    let left_max_w = left.lines().map(ansi_width::ansi_width).max().unwrap_or(0);

    let gap = 3;

    for i in left.lines() {
        s.push_str(i);
        let n = left_max_w + gap - ansi_width::ansi_width(i);
        s.push_str(&" ".repeat(n));
        s.push('\n');
    }

    for _ in left_h..max_h {
        s.push_str(&" ".repeat(left_max_w + gap));
        s.push('\n');
    }

    s.push_str(&cursor_up(max_h));
    for i in right.lines() {
        let line = format!("{}{i}", cursor_forward(left_max_w + gap));
        s.push_str(&line);
        s.push('\n');
    }
    s.push_str(&cursor_down(max_h - right_h));
    s
}

#[derive(Debug, Clone)]
pub struct Neofetch {
    pub os: Option<OS>,
    pub user: Option<String>,
    pub host: Option<String>,
    pub hostname: Option<String>,
    pub rom: Option<String>,
    pub baseband: Option<String>,
    pub kernel: Option<String>,
    pub uptime: Option<Time>,
    pub packages: Option<Packages>,
    pub shell: Option<ShellVersion>,
    pub display: Option<Vec<Display>>,
    pub de: Option<String>,
    pub wm: Option<String>,
    pub wm_theme: Option<String>,
    pub terminal: Option<String>,
    pub disk: Option<Vec<Disk>>,
    pub cpu: Option<Cpu>,
    pub gpu: Option<Vec<Gpu>>,
    pub memory: Option<String>,
    pub battery: Option<u32>,
    pub locale: Option<String>,
    pub ip: Option<String>,
}

impl Neofetch {
    pub async fn new() -> Neofetch {
        let (
            shell,
            os,
            user,
            host,
            rom,
            baseband,
            kernel,
            uptime,
            packages,
            display,
            wm,
            wm_theme,
            terminal,
            disk,
            cpu,
            gpu,
            memory,
            battery,
            hostname,
            locale,
        ) = tokio::join!(
            which_shell(),
            get_os(),
            get_user(),
            get_host(),
            get_rom(),
            get_baseband(),
            get_kernel(),
            get_uptime(),
            get_packages(),
            get_display(),
            get_wm(),
            get_wm_theme(),
            get_terminal(),
            get_disk(),
            get_cpu(),
            get_gpu(),
            get_memory(),
            get_battery(),
            get_hostname(),
            get_locale(),
        );
        let de = os.clone().and_then(get_de);
        let ip = ip::get_ip();
        Neofetch {
            os,
            user,
            host,
            rom,
            baseband,
            kernel,
            uptime,
            packages,
            shell,
            display,
            de,
            wm,
            wm_theme,
            terminal,
            disk,
            cpu,
            gpu,
            memory,
            battery,
            hostname,
            locale,
            ip,
        }
    }
}

impl std::fmt::Display for Neofetch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut info: String = String::new();
        let mut icon = String::new();
        let user = self.user.clone().unwrap_or_default();
        let hostname = self.hostname.clone().unwrap_or_default();

        info.push_str(&format!(
            "{RESET}{RED}{BOLD}{user}{RESET}@{RED}{BOLD}{hostname}{RESET}\n"
        ));
        info.push_str("-------\n");
        if let Some(os) = &self.os {
            let s = os.to_string();
            icon = os.distro.icon();
            info.push_str(&format!("{GREEN}{BOLD}OS: {RESET}{s}\n"));
        }

        if let Some(host) = &self.host {
            info.push_str(&format!("{GREEN}{BOLD}Host: {RESET}{host}\n"));
        }
        if let Some(rom) = &self.rom {
            info.push_str(&format!("{GREEN}{BOLD}Rom: {RESET}{rom}\n"));
        }

        if let Some(baseband) = &self.baseband {
            info.push_str(&format!("{GREEN}{BOLD}Baseband: {RESET}{baseband}\n"));
        }

        if let Some(kernel) = &self.kernel {
            info.push_str(&format!("{GREEN}{BOLD}Kernel: {RESET}{kernel}\n"));
        }

        if let Some(uptime) = self.uptime
            && uptime.0 > 0
        {
            info.push_str(&format!("{GREEN}{BOLD}Uptime: {RESET}{uptime}\n"));
        }
        if let Some(packages) = &self.packages {
            let s = packages.to_string();
            if !s.trim().is_empty() {
                info.push_str(&format!("{GREEN}{BOLD}Packages: {RESET}{s}\n"));
            }
        }
        if let Some(shell) = &self.shell {
            info.push_str(&format!("{GREEN}{BOLD}Shell: {RESET}{shell}\n"));
        }

        if let Some(displays) = &self.display {
            for display in displays {
                let key = if let Some(i) = &display.friendly_name {
                    format!("{GREEN}{BOLD}Display({i})")
                } else {
                    display
                        .name
                        .clone()
                        .map_or(format!("{GREEN}{BOLD}Display"), |s| {
                            format!("{GREEN}{BOLD}Display({s})")
                        })
                };

                info.push_str(&format!("{key}: {RESET}{}\n", &display));
            }
        }

        if let Some(de) = &self.de {
            info.push_str(&format!("{GREEN}{BOLD}DE: {RESET}{de}\n"));
        }

        if let Some(wm) = &self.wm {
            info.push_str(&format!("{GREEN}{BOLD}WM: {RESET}{wm}\n"));

            if let Some(theme) = &self.wm_theme {
                info.push_str(&format!("{GREEN}{BOLD}WM Theme: {RESET}{theme}\n"));
            }
        }

        if let Some(terminal) = &self.terminal {
            info.push_str(&format!("{GREEN}{BOLD}Terminal: {RESET}{terminal}\n"));
        }

        if let Some(disks) = &self.disk
            && !disks.is_empty()
        {
            for disk in disks {
                if disk.total > 0 {
                    info.push_str(&format!(
                        "{GREEN}{BOLD}Disk({}): {RESET}{}\n",
                        disk.name, disk
                    ));
                }
            }
        }

        if let Some(cpu) = &self.cpu {
            info.push_str(&format!("{GREEN}{BOLD}CPU: {RESET}{cpu}\n"));
        }

        if let Some(gpu) = &self.gpu {
            for i in gpu {
                info.push_str(&format!("{GREEN}{BOLD}GPU: {RESET}{i}\n"));
            }
        }

        if let Some(memory) = &self.memory {
            info.push_str(&format!("{GREEN}{BOLD}Memory: {RESET}{memory}\n"));
        }

        if let Some(battery) = &self.battery {
            info.push_str(&format!("{GREEN}{BOLD}Battery: {RESET}{battery}\n"));
        }

        if let Some(ip) = &self.ip {
            info.push_str(&format!("{GREEN}{BOLD}Local IP: {RESET}{ip}\n"));
        }

        if let Some(locale) = &self.locale {
            info.push_str(&format!("{GREEN}{BOLD}Locale: {RESET}{locale}\n"));
        }
        let color_str: String = [
            BLACK_BG, RED_BG, GREEN_BG, YELLOW_BG, BLUE_BG, MAGENTA_BG, CYAN_BG, WHITE_BG,
        ]
        .map(|c| format!("{c}   "))
        .into_iter()
        .collect();
        info.push('\n');

        info.push_str(&(color_str + RESET + "\n"));
        let color_str: String = [
            BRIGHT_BLACK_BG,
            BRIGHT_RED_BG,
            BRIGHT_GREEN_BG,
            BRIGHT_YELLOW_BG,
            BRIGHT_BLUE_BG,
            BRIGHT_MAGENTA_BG,
            BRIGHT_CYAN_BG,
            BRIGHT_WHITE_BG,
        ]
        .map(|c| format!("{c}   "))
        .into_iter()
        .collect();
        info.push_str(&(color_str + RESET + "\n"));

        f.write_str(&join(icon, info))
    }
}

pub async fn neofetch() -> String {
    Neofetch::new().await.to_string()
}
