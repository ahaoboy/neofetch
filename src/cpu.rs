use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub name: String,
    pub cores: u32,
    pub speed: u32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} ({}) {:.2} GHz", self.name, self.cores, self.speed as f64 / 1000.))
    }
}

#[cfg(windows)]
pub async fn get_cpu() -> Option<Vec<Cpu>> {
    use crate::share::wmi_query;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename = "Win32_Processor")]
    pub struct Processor {
        #[serde(rename = "Name")]
        pub name: String,
        // NumberOfLogicalProcessors
        // NumberOfCores
        #[serde(rename = "NumberOfLogicalProcessors")]
        pub number_of_cores: u32,
        #[serde(rename = "CurrentClockSpeed")]
        pub current_clock_speed: u32,
    }

    let results: Vec<Processor> = wmi_query().await?;
    Some(
        results
            .iter()
            .map(|i| Cpu {
                name: i.name.clone(),
                cores: i.number_of_cores,
                speed: i.current_clock_speed,
            })
            .collect(),
    )
}

#[cfg(unix)]
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
