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
pub mod kernel;
pub mod memory;
pub mod os;
pub mod packages;
pub mod resolution;
pub mod share;
pub mod shell;
pub mod terminal;
pub mod uptime;
pub mod user;
pub mod wm;

use display::get_display;
use icon::Darwin;

use crate::battery::get_battery;
use crate::color::{
    cursor_down, cursor_forward, cursor_up, BLACK_BG, BLUE_BG, BOLD, BRIGHT_BLACK_BG,
    BRIGHT_BLUE_BG, BRIGHT_CYAN_BG, BRIGHT_GREEN_BG, BRIGHT_MAGENTA_BG, BRIGHT_RED_BG,
    BRIGHT_WHITE_BG, BRIGHT_YELLOW_BG, CYAN_BG, GREEN, GREEN_BG, MAGENTA_BG, RED, RED_BG, RESET,
    WHITE_BG, YELLOW_BG,
};
use crate::cpu::get_cpu;
use crate::de::get_de;
use crate::disk::get_disk;
use crate::host::get_host;
use crate::host::{get_baseband, get_rom};
use crate::hostname::get_hostname;
use crate::icon::{Android, Linux, Ubuntu};
use crate::icon::{Windows, Windows_10, Windows_11};
use crate::kernel::get_kernel;
use crate::memory::get_memory;
use crate::packages::get_packages;
use crate::shell::get_shell;
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

pub fn neofetch() -> String {
    let mut info: String = String::new();
    let mut icon = String::new();
    let user = get_user().unwrap_or_default();
    let hostname = get_hostname().unwrap_or_default();

    info.push_str(&format!(
        "{RED}{BOLD}{user}{RESET}@{RED}{BOLD}{hostname}{RESET}\n"
    ));
    info.push_str("-------\n");
    if let Some(os) = get_os() {
        let s = os.to_string();
        info.push_str(&format!("{GREEN}{BOLD}OS: {RESET}{s}\n"));
        if s.starts_with("Windows 11") {
            icon = Windows_11()
        } else if s.starts_with("Windows 10") {
            icon = Windows_10()
        } else if s.starts_with("Windows") {
            icon = Windows()
        } else if s.starts_with("Android") {
            icon = Android()
        } else if s.starts_with("Ubuntu") {
            icon = Ubuntu()
        } else if s.starts_with("Linux") {
            icon = Linux()
        } else if s.starts_with("Darwin") {
            icon = Darwin()
        }
    }

    if let Some(host) = get_host() {
        info.push_str(&format!("{GREEN}{BOLD}Host: {RESET}{host}\n"));
    }
    if let Some(rom) = get_rom() {
        info.push_str(&format!("{GREEN}{BOLD}Rom: {RESET}{rom}\n"));
    }

    if let Some(baseband) = get_baseband() {
        info.push_str(&format!("{GREEN}{BOLD}Baseband: {RESET}{baseband}\n"));
    }

    if let Some(kernel) = get_kernel() {
        info.push_str(&format!("{GREEN}{BOLD}Kernel: {RESET}{kernel}\n"));
    }

    if let Some(uptime) = get_uptime() {
        info.push_str(&format!("{GREEN}{BOLD}Uptime: {RESET}{uptime}\n"));
    }
    if let Some(packages) = get_packages() {
        let s = packages.to_string();
        if !s.trim().is_empty() {
            info.push_str(&format!("{GREEN}{BOLD}Packages: {RESET}{}\n", s));
        }
    }
    if let Some(shell) = get_shell() {
        info.push_str(&format!("{GREEN}{BOLD}Shell: {RESET}{}\n", shell));
    }

    if let Some(displays) = get_display() {
        for display in displays {
            let key = display
                .name
                .clone()
                .map_or(format!("{GREEN}{BOLD}Display"), |s| {
                    format!("{GREEN}{BOLD}Display({s})")
                });
            info.push_str(&format!("{key}: {RESET}{}\n", display));
        }
    }

    // if let Some(resolution) = get_resolution() {
    //     info.push_str(&format!("{GREEN}{BOLD}Resolution: {RESET}{resolution}\n"));
    // }

    if let Some(de) = get_de() {
        info.push_str(&format!("{GREEN}{BOLD}DE: {RESET}{de}\n"));
    }

    if let Some(wm) = get_wm() {
        info.push_str(&format!("{GREEN}{BOLD}WM: {RESET}{wm}\n"));

        if let Some(theme) = get_wm_theme() {
            info.push_str(&format!("{GREEN}{BOLD}WM Theme: {RESET}{theme}\n"));
        }
    }

    if let Some(terminal) = get_terminal() {
        info.push_str(&format!("{GREEN}{BOLD}Terminal: {RESET}{terminal}\n"));
    }

    if let Some(disks) = get_disk() {
        if !disks.is_empty() {
            for disk in disks {
                info.push_str(&format!(
                    "{GREEN}{BOLD}Disk({}): {RESET}{}\n",
                    disk.name, disk
                ));
            }
        }
    }

    if let Some(cpu) = get_cpu() {
        info.push_str(&format!("{GREEN}{BOLD}CPU: {RESET}{cpu}\n"));
    }

    if let Some(gpu) = get_gpu() {
        info.push_str(&format!("{GREEN}{BOLD}GPU: {RESET}{gpu}\n"));
    }

    if let Some(memory) = get_memory() {
        info.push_str(&format!("{GREEN}{BOLD}Memory: {RESET}{memory}\n"));
    }

    if let Some(battery) = get_battery() {
        info.push_str(&format!("{GREEN}{BOLD}Battery: {RESET}{battery}\n"));
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

    join(icon, info)
}
