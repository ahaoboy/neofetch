use crate::share::exec;

pub fn get_cpu() -> Option<String> {
    let cpu = exec(
        "awk",
        ["-F", ":", "/model name/ {print $2; exit}", "/proc/cpuinfo"],
    )?;

    let feq = exec(
        "awk",
        ["-F", ":", "/cpu MHz/ {printf $2; exit}", "/proc/cpuinfo"],
    )?;
    let feq: f64 = feq.parse().ok()?;
    let feq_str = if feq < 1000. {
        format!("{:.2}MHz", feq)
    } else {
        format!("{:.2}GHz", feq / 1000.)
    };

    Some(format!("{cpu} @ {feq_str}"))
}