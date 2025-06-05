use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub name: String,
    pub cores: u32,
    pub speed: u32,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];

        v.push(self.name.clone());
        if self.cores > 0 {
            v.push(format!("({})", self.cores));
        }
        if self.speed > 0 {
            v.push(format!("{:.2} GHz", self.speed as f64 / 1000.));
        }
        let s = v.join(" ");
        f.write_str(&s)
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
pub async fn get_cpu() -> Option<Vec<Cpu>> {
    use std::collections::HashMap;

    let s = tokio::fs::read_to_string("/proc/cpuinfo").await.ok()?;
    let mut cpuinfo = HashMap::new();
    for line in s.lines() {
        let mut line = line.split(':');

        if let (Some(key), Some(value)) = (line.next(), line.next()) {
            cpuinfo.insert(key.trim(), value.trim());
        }
    }

    let name = cpuinfo.get("model name").or(cpuinfo.get("Hardware"))?;

    let mut cpu = Cpu {
        name: name.to_string(),
        cores: 0,
        speed: 0,
    };

    if let Some(Some(n)) = cpuinfo.get("cpu MHz").map(|s| s.parse::<f64>().ok()) {
        cpu.speed = n as u32;
    }
    if let Some(Some(n)) = cpuinfo.get("cpu cores").map(|s| s.parse::<f64>().ok()) {
        cpu.cores = n as u32;
    }
    Some(vec![cpu])
}
