use crate::share::exec;
use std::fmt::Display;
const ONE_MINUTE: u64 = 60;
const ONE_HOUR: u64 = 60 * 60;
const ONE_DAY: u64 = 60 * 60 * 24;

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

pub fn get_uptime() -> Option<Time> {
    let mut time: u64 = 0;
    if let Some(uptime) = exec("cat", ["/proc/uptime"]) {
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
