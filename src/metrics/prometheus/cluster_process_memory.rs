use crate::{
    metrics::MetricsConvertible, status_models::cluster_process_memory::ClusterProcessMemory,
};
use lazy_static::lazy_static;
use prometheus::{register_int_gauge_vec, IntGaugeVec};

lazy_static! {
    static ref P_PROCESS_MEMORY_AVAILABLE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_available_bytes",
        "Available bytes for the current process",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_LIMIT_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_limit_bytes",
        "Limiting bytes for the current process",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_RSS_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_rss_bytes",
        "N/A",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_UNUSED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_unused_allocated_bytes",
        "N/A",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_USED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_used_bytes",
        "N/A",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
}

impl MetricsConvertible for ClusterProcessMemory {
    fn to_metrics(&self, labels: &[&str]) {
        P_PROCESS_MEMORY_AVAILABLE_BYTES
            .with_label_values(labels)
            .set(self.available_bytes);
        P_PROCESS_MEMORY_LIMIT_BYTES
            .with_label_values(labels)
            .set(self.limit_bytes);
        P_PROCESS_MEMORY_RSS_BYTES
            .with_label_values(labels)
            .set(self.rss_bytes);
        P_PROCESS_MEMORY_UNUSED_BYTES
            .with_label_values(labels)
            .set(self.unused_allocated_memory);
        P_PROCESS_MEMORY_USED_BYTES
            .with_label_values(labels)
            .set(self.used_bytes);
    }
}
