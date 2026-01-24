use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStats {
    pub cpu_stats: CpuStats,
    pub memory_stats: MemoryStats,
    pub precpu_stats: CpuStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: Option<u64>,
    pub online_cpus: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuUsage {
    pub total_usage: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryStats {
    pub usage: u64,
    pub stats: MemoryInfo,
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryInfo {
    pub file: u64,
}
