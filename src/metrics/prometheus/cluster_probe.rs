use lazy_static::lazy_static;
use prometheus::{register_gauge, Gauge};

use crate::{metrics::MetricsConvertible, status_models::cluster_probe::ClusterLatencyProbe};

lazy_static! {
    static ref P_CLUSTER_LATENCY_PROBE_COMMIT_SECONDS: Gauge = register_gauge!(
        "fdb_cluster_latency_commit_seconds",
        "Time in seconds to commit a transaction"
    )
    .unwrap();
    static ref P_CLUSTER_LATENCY_READ_SECONDS: Gauge = register_gauge!(
        "fdb_cluster_latency_read_seconds",
        "Time in seconds to read"
    )
    .unwrap();
    static ref P_CLUSTER_LATENCY_TRANSACTION_START_SECONDS: Gauge = register_gauge!(
        "fdb_cluster_latency_transaction_start_seconds",
        "Time in seconds to start a transaction"
    )
    .unwrap();
    static ref P_CLUSTER_LATENCY_IMMEDIATE_PRIORITY_START_SECONDS: Gauge = register_gauge!(
        "fdb_cluster_latency_immediate_priority_start_seconds",
        "N/A"
    )
    .unwrap();
}

impl MetricsConvertible for ClusterLatencyProbe {
    fn to_metrics(&self, _: &[&str]) {
        if let Some(commit_seconds) = self.commit_seconds {
            P_CLUSTER_LATENCY_PROBE_COMMIT_SECONDS.set(commit_seconds);
        }
        if let Some(read_seconds) = self.read_seconds {
            P_CLUSTER_LATENCY_READ_SECONDS.set(read_seconds);
        }
        if let Some(transaction_start_seconds) = self.transaction_start_seconds {
            P_CLUSTER_LATENCY_TRANSACTION_START_SECONDS.set(transaction_start_seconds);
        }
        if let Some(immediate_priority_start_seconds) = self.immediate_priority_start_seconds {
            P_CLUSTER_LATENCY_IMMEDIATE_PRIORITY_START_SECONDS
                .set(immediate_priority_start_seconds);
        }
    }
}
