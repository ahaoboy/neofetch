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
pub async fn get_uptime() -> Option<Time> {
    use crate::share::wmi_query;
    use chrono::TimeZone;
    use chrono::Utc;
    use chrono::{FixedOffset, NaiveDateTime};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_OperatingSystem")]
    struct OperatingSystem {
        #[serde(rename = "LastBootUpTime")]
        last_boot_up_time: String,
    }

    let results: Vec<OperatingSystem> = wmi_query().await?;
    // 20250530024623.265456+480
    let input = results.first().map(|i| i.last_boot_up_time.clone())?;

    let now = Utc::now();
    let datetime_str = &input[..21]; // "20250530024623.265456"
    let offset_str = &input[21..]; // "+480"

    let naive_dt = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M%S%.f").ok()?;

    let offset_minutes: i32 = offset_str.parse::<i32>().ok()?;
    let offset = FixedOffset::east_opt(offset_minutes * 60)?;

    let datetime_with_tz = offset
        .from_local_datetime(&naive_dt)
        .unwrap()
        .with_timezone(&Utc);

    let uptime = now - datetime_with_tz;
    let uptime_seconds = uptime.num_seconds() as u64;

    Some(Time(uptime_seconds))
}

#[cfg(unix)]
pub async fn get_uptime() -> Option<Time> {
    let mut time: u64 = 0;
    if let Some(uptime) = exec_async("cat", ["/proc/uptime"]).await {
        if !uptime.is_empty() {
            let s = uptime.split(' ').next().unwrap_or_default();
            time = s.parse::<f64>().ok()? as u64;
        }
    }

    if let Some(uptime) = exec("uptime", ["-s"]) {
        if let (Some(boot), Some(now)) = (
            exec("date", ["-d", uptime.as_str(), "+%s"]),
            exec("date", ["+%s"]),
        ) {
            if let (Ok(boot), Ok(now)) = (boot.parse::<f64>(), now.parse::<f64>()) {
                time = (now - boot) as u64;
            }
        }
    }

    Some(Time(time))
}
