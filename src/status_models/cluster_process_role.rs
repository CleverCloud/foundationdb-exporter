use serde::Deserialize;

use super::cluster_process::ClusterClassType;

#[derive(Deserialize)]
pub struct RoleId(pub String);

// jq: .cluster.processes[].roles[]
#[derive(Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct ClusterProcessRole {
    pub query_queue_max: Option<f64>,
    pub local_rate: Option<f64>,
    pub stored_bytes: Option<i64>,

    pub kvstore_used_bytes: Option<i64>,
    pub kvstore_available_bytes: Option<i64>,
    pub kvstore_free_bytes: Option<i64>,
    pub kvstore_total_bytes: Option<i64>,
    pub kvstore_total_size: Option<i64>,
    pub kvstore_total_nodes: Option<i64>,
    pub kvstore_inline_keys: Option<i64>,

    pub queue_disk_used_bytes: Option<i64>,
    pub queue_disk_available_bytes: Option<i64>,
    pub queue_disk_free_bytes: Option<i64>,
    pub queue_disk_total_bytes: Option<i64>,

    pub role: Option<ClusterClassType>,

    pub data_version: Option<i64>,
    pub durable_version: Option<i64>,

    pub data_lag: Option<DataLag>,
    pub durability_lag: Option<DataLag>,

    pub id: Option<RoleId>,

    pub durable_bytes: Option<ClusterProcessRoleFreq>,
    pub input_bytes: Option<ClusterProcessRoleFreq>,

    pub total_queries: Option<ClusterProcessRoleFreq>,
    pub finished_queries: Option<ClusterProcessRoleFreq>,
    pub low_priority_queries: Option<ClusterProcessRoleFreq>,
    pub bytes_queried: Option<ClusterProcessRoleFreq>,
    pub keys_queried: Option<ClusterProcessRoleFreq>,
    pub mutation_bytes: Option<ClusterProcessRoleFreq>,
    pub mutations: Option<ClusterProcessRoleFreq>,
    pub fetched_versions: Option<ClusterProcessRoleFreq>,
    pub fetches_from_logs: Option<ClusterProcessRoleFreq>,

    pub grv_latency_statistics: Option<ClusterProcessRoleGrvLatency>,
    pub read_latency_statistics: Option<LatencyStats>,
    pub commit_latency_statistics: Option<LatencyStats>,
    pub commit_batching_window_size: Option<LatencyStats>,
}

// jq: .cluster.processes[].roles[].grv_latency_statistics
#[derive(Deserialize)]
pub struct ClusterProcessRoleGrvLatency {
    pub default: Option<LatencyStats>,
    pub batch: Option<LatencyStats>,
}

#[derive(Deserialize)]
pub struct LatencyStats {
    pub count: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub mean: f64,
    pub p25: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    #[serde(alias = "p99.9")]
    pub p99_9: f64,
}

#[derive(Deserialize)]
pub struct DataLag {
    pub seconds: f64,
    pub versions: i64,
}

#[derive(Deserialize)]
pub struct ClusterProcessRoleFreq {
    pub counter: i64,
    pub hz: f64,
    pub roughness: f64,
}
