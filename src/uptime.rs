const ONE_MINUTE: u64 = 60;
const ONE_HOUR: u64 = 60 * 60;
const ONE_DAY: u64 = 60 * 60 * 24;
use crate::share::exec;

fn with_unit(n: u64, unit: &str) -> String {
    format!("{n} {unit}{}", if n > 1 { "s" } else { "" })
}

pub fn get_uptime() -> Option<String> {
    let s = exec("cat", ["/proc/uptime"])?;

    let time = s.trim().split(' ').next()?;
    let time: f64 = time.parse().ok()?;
    let time: u64 = time as u64;
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
        with_unit(day, "hour"),
        with_unit(hour, "hour"),
        with_unit(min, "min")
    ))
}
