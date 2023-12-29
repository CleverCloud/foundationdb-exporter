use serde::Deserialize;

/// jq: .cluster.processes[].disk
#[derive(Deserialize)]
pub struct ClusterProcessDisk {
    pub busy: f64,
    pub free_bytes: i64,
    pub total_bytes: i64,
    pub reads: ClusterProcessDiskStat,
    pub writes: ClusterProcessDiskStat,
}

// jq: .cluster.processes[].disk.{reads, writes}
#[derive(Deserialize)]
pub struct ClusterProcessDiskStat {
    pub counter: i64,
    pub hz: f64,
    pub sectors: f64,
}
