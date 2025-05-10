use std::collections::HashMap;
use tracing::instrument;

use crate::share::exec_async;

#[instrument]
pub async fn get_cpu() -> Option<String> {
    let s = exec_async("cat", ["/proc/cpuinfo"]).await?;
    let mut cpuinfo = HashMap::new();
    for line in s.lines() {
        let mut line = line.split(':');

        if let (Some(key), Some(value)) = (line.next(), line.next()) {
            cpuinfo.insert(key.trim(), value.trim());
        }
    }

    let cpu = cpuinfo.get("model name").or(cpuinfo.get("Hardware"))?;

    let feq = cpuinfo.get("cpu MHz").map(|s| s.parse::<f64>().ok());

    if let Some(Some(feq)) = feq {
        let feq_str = if feq < 1000. {
            format!("{:.2}MHz", feq)
        } else {
            format!("{:.2}GHz", feq / 1000.)
        };

        return Some(format!("{cpu} @ {feq_str}"));
    }

    Some(cpu.to_string())
}
