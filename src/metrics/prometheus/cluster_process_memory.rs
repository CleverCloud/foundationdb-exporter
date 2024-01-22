use crate::metrics::prometheus::PROCESS_LABELS;
use crate::{
    metrics::MetricsConvertible, status_models::cluster_process_memory::ClusterProcessMemory,
};
use lazy_static::lazy_static;
use prometheus::{register_int_gauge_vec, IntGaugeVec};

lazy_static! {
    static ref P_PROCESS_MEMORY_AVAILABLE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_available_bytes",
        "Available bytes for the current process",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_LIMIT_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_limit_bytes",
        "Limiting bytes for the current process",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_RSS_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_rss_bytes",
        "N/A",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_UNUSED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_unused_allocated_bytes",
        "N/A",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_MEMORY_USED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_memory_used_bytes",
        "N/A",
        PROCESS_LABELS,
    )
    .unwrap();
}

impl MetricsConvertible for ClusterProcessMemory {
    fn to_metrics(&self, labels: &[&str]) {
        if let Some(available_bytes) = self.available_bytes {
            P_PROCESS_MEMORY_AVAILABLE_BYTES
                .with_label_values(labels)
                .set(available_bytes);
        }
        if let Some(limit_bytes) = self.limit_bytes {
            P_PROCESS_MEMORY_LIMIT_BYTES
                .with_label_values(labels)
                .set(limit_bytes);
        }
        if let Some(rss_bytes) = self.rss_bytes {
            P_PROCESS_MEMORY_RSS_BYTES
                .with_label_values(labels)
                .set(rss_bytes);
        }

        if let Some(unused_allocated_memory) = self.unused_allocated_memory {
            P_PROCESS_MEMORY_UNUSED_BYTES
                .with_label_values(labels)
                .set(unused_allocated_memory);
        }
        if let Some(used_bytes) = self.used_bytes {
            P_PROCESS_MEMORY_USED_BYTES
                .with_label_values(labels)
                .set(used_bytes);
        }
    }
}
