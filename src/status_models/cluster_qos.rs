use serde::Deserialize;

use super::{cluster_process::ProcessId, cluster_process_role::DataLag};

/// jq: .cluster.qos
#[derive(Deserialize)]
pub struct ClusterQos {
    pub worst_queue_bytes_log_server: i64,
    pub worst_queue_bytes_storage_server: i64,
    pub limiting_queue_bytes_storage_server: i64,

    pub batch_transactions_per_second_limit: f64,
    pub transactions_per_second_limit: f64,
    pub batch_released_transactions_per_second: f64,
    pub released_transactions_per_second: f64,

    pub limiting_data_lag_storage_server: Option<DataLag>,
    pub limiting_durability_lag_storage_server: Option<DataLag>,
    pub worst_data_lag_storage_server: Option<DataLag>,
    pub worst_durability_lag_storage_server: Option<DataLag>,

    pub batch_performance_limited_by: ClusterPerformanceLimit,
    pub performance_limited_by: ClusterPerformanceLimit,
}

#[derive(Deserialize)]
pub struct ClusterPerformanceLimit {
    pub reason_server_id: Option<ProcessId>,
    pub reason_id: i64,
    pub name: String,
    pub description: String,
}
