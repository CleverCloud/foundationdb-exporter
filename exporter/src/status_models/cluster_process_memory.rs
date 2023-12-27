use serde::Deserialize;

/// jq: .cluster.processes[].memory
#[derive(Deserialize)]
pub struct ClusterProcessMemory {
    pub available_bytes: i64,
    pub limit_bytes: i64,
    pub rss_bytes: i64,
    pub unused_allocated_memory: i64,
    pub used_bytes: i64,
}
