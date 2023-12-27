use crate::metrics::MetricsConvertible;
use crate::status_models::cluster::ClusterData;
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};

lazy_static! {
    static ref P_CLUSTER_AVG_PARTITION_BYTES_GAUGE: IntGauge = register_int_gauge!(
        "fdb_cluster_average_partition_size_bytes",
        "Average size for a partition in the cluster"
    )
    .unwrap();
    static ref P_CLUSTER_LEAST_SPACE_BYTES_LOG_SERVER_GAUGE: IntGauge = register_int_gauge!(
        "fdb_cluster_least_space_log_server_bytes",
        "Value of the log server with least space available"
    )
    .unwrap();
    static ref P_CLUSTER_LEAST_SPACE_BYTES_STORAGE_SERVER: IntGauge = register_int_gauge!(
        "fdb_cluster_least_space_storage_server_bytes",
        "Value of the storage server with least space avaiable"
    )
    .unwrap();
    static ref P_CLUSTER_PARTITION_COUNT: IntGauge =
        register_int_gauge!("fdb_cluster_partition_count", "Number of partitions").unwrap();
    static ref P_CLUSTER_TOTAL_DISK_USED_BYTES: IntGauge = register_int_gauge!(
        "fdb_cluster_total_disk_used_bytes",
        "Total number of bytes used on all disk"
    )
    .unwrap();
    static ref P_CLUSTER_TOTAL_KV_SIZE_BYTES: IntGauge = register_int_gauge!(
        "fdb_cluster_total_kv_size_bytes",
        "Total number of bytes for all key values"
    )
    .unwrap();
    static ref P_CLUSTER_STATE_HEALTHY: IntGauge = register_int_gauge!(
        "fdb_cluster_healthy",
        "Whether the cluster is healthy or not"
    )
    .unwrap();
}

impl MetricsConvertible for ClusterData {
    fn to_metrics(&self, _: &[&str]) {
        P_CLUSTER_TOTAL_KV_SIZE_BYTES.set(self.total_kv_size_bytes);
        P_CLUSTER_TOTAL_DISK_USED_BYTES.set(self.total_disk_used_bytes);
        P_CLUSTER_PARTITION_COUNT.set(self.partitions_count);
        P_CLUSTER_LEAST_SPACE_BYTES_STORAGE_SERVER
            .set(self.least_operating_space_bytes_storage_server);
        P_CLUSTER_LEAST_SPACE_BYTES_LOG_SERVER_GAUGE
            .set(self.least_operating_space_bytes_log_server);
        P_CLUSTER_AVG_PARTITION_BYTES_GAUGE.set(self.average_partition_size_bytes);
        P_CLUSTER_STATE_HEALTHY.set(self.state.healthy as i64);
    }
}
