const ONE_MINUTE: u64 = 60;
const ONE_HOUR: u64 = 60 * 60;
const ONE_DAY: u64 = 60 * 60 * 24;
use crate::share::exec;

fn with_unit(n: u64, unit: &str) -> String {
    format!("{n} {unit}{}", if n > 1 { "s" } else { "" })
}

pub fn get_uptime() -> Option<String> {
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

    if time < ONE_MINUTE {
        return Some(with_unit(time, "sec"));
    }
    if time < ONE_HOUR {
        let min = time / ONE_MINUTE;
        let sec = time - min * ONE_MINUTE;
        return Some(format!(
            "{} {}",
            with_unit(min, "min"),
            with_unit(sec, "sec")
        ));
    }

    if time < ONE_DAY {
        let hour = time / ONE_HOUR;
        let min = (time - hour * ONE_HOUR) / ONE_MINUTE;
        let sec = time - hour * ONE_HOUR - min * ONE_MINUTE;
        return Some(format!(
            "{}, {}, {}",
            with_unit(hour, "hour"),
            with_unit(min, "min"),
            with_unit(sec, "sec")
        ));
    }
    let day = time / ONE_DAY;
    let hour = (time - day * ONE_DAY) / ONE_HOUR;
    let min = (time - day * ONE_DAY - hour * ONE_HOUR) / ONE_MINUTE;
    Some(format!(
        "{}, {}, {}",
        with_unit(day, "day"),
        with_unit(hour, "hour"),
        with_unit(min, "min")
    ))
}
