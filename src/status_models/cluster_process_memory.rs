use serde::Deserialize;

/// jq: .cluster.processes[].memory
#[derive(Deserialize)]
pub struct ClusterProcessMemory {
    pub available_bytes: Option<i64>,
    pub limit_bytes: Option<i64>,
    pub rss_bytes: Option<i64>,
    pub unused_allocated_memory: Option<i64>,
    pub used_bytes: Option<i64>,
}
