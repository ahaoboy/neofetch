use std::fmt::Display;
const ONE_MINUTE: u64 = 60;
const ONE_HOUR: u64 = 60 * 60;
const ONE_DAY: u64 = 60 * 60 * 24;

#[derive(Debug, Clone, Copy)]
pub struct Time(pub u64);

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < ONE_MINUTE {
            return f.write_str(&with_unit(self.0, "sec"));
        }

        if self.0 < ONE_HOUR {
            let min = self.0 / ONE_MINUTE;
            let sec = self.0 - min * ONE_MINUTE;
            return f.write_str(&format!(
                "{} {}",
                with_unit(min, "min"),
                with_unit(sec, "sec")
            ));
        }

        if self.0 < ONE_DAY {
            let hour = self.0 / ONE_HOUR;
            let min = (self.0 - hour * ONE_HOUR) / ONE_MINUTE;
            let sec = self.0 - hour * ONE_HOUR - min * ONE_MINUTE;
            return f.write_str(&format!(
                "{}, {}, {}",
                with_unit(hour, "hour"),
                with_unit(min, "min"),
                with_unit(sec, "sec")
            ));
        }
        let day = self.0 / ONE_DAY;
        let hour = (self.0 - day * ONE_DAY) / ONE_HOUR;
        let min = (self.0 - day * ONE_DAY - hour * ONE_HOUR) / ONE_MINUTE;
        f.write_str(&format!(
            "{}, {}, {}",
            with_unit(day, "day"),
            with_unit(hour, "hour"),
            with_unit(min, "min")
        ))
    }
}

fn with_unit(n: u64, unit: &str) -> String {
    format!("{n} {unit}{}", if n > 1 { "s" } else { "" })
}

#[cfg(windows)]
pub async fn get_uptime() -> crate::error::Result<Time> {
    use chrono::TimeZone;
    use chrono::Utc;
    use chrono::{FixedOffset, NaiveDateTime};
    use serde::Deserialize;

    use crate::platform::wmi_query;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "LastBootUpTime")]
        last_boot_up_time: String,
    }

    let results: Vec<OperatingSystem> = wmi_query().await?;
    // Format: 20250530024623.265456+480
    let input = results
        .first()
        .map(|i| i.last_boot_up_time.clone())
        .ok_or_else(|| crate::error::NeofetchError::data_unavailable("Boot time not found"))?;

    let now = Utc::now();

    if input.len() < 21 {
        return Err(crate::error::NeofetchError::parse_error(
            "boot_time",
            "Invalid boot time format",
        ));
    }

    let datetime_str = &input[..21]; // "20250530024623.265456"
    let offset_str = &input[21..]; // "+480"

    let naive_dt = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M%S%.f")
        .map_err(|e| crate::error::NeofetchError::parse_error("boot_time", e.to_string()))?;

    let offset_minutes: i32 = offset_str.parse().map_err(|e| {
        crate::error::NeofetchError::parse_error("timezone_offset", format!("{}", e))
    })?;

    let offset = FixedOffset::east_opt(offset_minutes * 60).ok_or_else(|| {
        crate::error::NeofetchError::parse_error("timezone_offset", "Invalid offset")
    })?;

    let datetime_with_tz = offset
        .from_local_datetime(&naive_dt)
        .unwrap()
        .with_timezone(&Utc);

    let uptime = now - datetime_with_tz;
    let uptime_seconds = uptime.num_seconds() as u64;

    Ok(Time(uptime_seconds))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub async fn get_uptime() -> crate::error::Result<Time> {
    let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
    let result = unsafe { libc::sysinfo(&mut info) };
    if result != 0 {
        return Err(crate::error::NeofetchError::system_call(
            "Failed to get system uptime from sysinfo",
        ));
    }
    Ok(Time(info.uptime as u64))
}

#[cfg(target_os = "macos")]
pub async fn get_uptime() -> crate::error::Result<Time> {
    let mut mib = [libc::CTL_KERN as i32, libc::KERN_BOOTTIME as i32];
    let mut boot_time: libc::timeval = unsafe { std::mem::zeroed() };
    let mut size = std::mem::size_of::<libc::timeval>();

    let result = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            2,
            &mut boot_time as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if result != 0 {
        return Err(crate::error::NeofetchError::system_call(
            "Failed to get boot time from sysctl",
        ));
    }

    // Get current time
    let mut current_time = unsafe { std::mem::zeroed() };
    let time_result = unsafe { libc::gettimeofday(&mut current_time, std::ptr::null_mut()) };
    if time_result != 0 {
        return Err(crate::error::NeofetchError::system_call(
            "Failed to get current time from gettimeofday",
        ));
    }

    // Calculate uptime in seconds
    let uptime = current_time.tv_sec - boot_time.tv_sec;
    Ok(Time(uptime as u64))
}
