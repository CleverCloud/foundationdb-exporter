use std::collections::HashMap;

use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};
use tracing::warn;

use crate::metrics::prometheus::PROCESS_LABELS;
use crate::{
    metrics::{prometheus::AndSet, MetricsConvertible},
    status_models::cluster_process_role::{
        ClusterProcessRole, ClusterProcessRoleFreq, LatencyStats,
    },
};

use super::StaticMetric;

lazy_static! {
    // KvStore
    static ref P_KVSTORE_USED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_kvstore_used_bytes",
        "KVStore used bytes",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_KVSTORE_AVAILABLE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_kvstore_available_bytes",
        "KVStore available bytes",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_KVSTORE_FREE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_kvstore_free_bytes",
        "KVStore free bytes",
        PROCESS_LABELS,
    ).unwrap();
    // Queue related
    static ref P_QUERY_QUEUE_MAX: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_role_queue_max",
        "Queue of read queries",
        PROCESS_LABELS,
    ).unwrap();
    static ref P_QUEUE_DISK_USED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_used_bytes",
        "Used bytes in the queue of a process",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_QUEUE_DISK_AVAILABLE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_available_bytes",
        "Available bytes in the queue of a process",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_QUEUE_DISK_FREE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_free_bytes",
        "Free bytes in the queue of a process",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_QUEUE_DISK_TOTAL_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_total_bytes",
        "Total bytes in the queue of a process",
        PROCESS_LABELS,
    ).unwrap();

    // Lag related
    static ref P_DATA_LAG_SECONDS: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_role_data_lag_seconds",
        "Lag in seconds of the process role",
        PROCESS_LABELS,
    ).unwrap();

    static ref P_DATA_DURABLE_LAG_SECONDS: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_role_durable_lag_seconds",
        "Lag in seconds of data being durable of the process role",
        PROCESS_LABELS,
    ).unwrap();

    // Latency related
    static ref P_DATA_READ_LATENCY: HashMap<String, GaugeVec> = LatencyStats::register("fdb_cluster_process_role_read_latency", "Latency of read");
    static ref P_DATA_COMMIT_LATENCY: HashMap<String, GaugeVec> = LatencyStats::register("fdb_cluster_process_role_commit_latency", "Latency for proxies");
    static ref P_DATA_COMMIT_BATCHING_WINDOW_SIZE: HashMap<String, GaugeVec> = LatencyStats::register("fdb_cluster_process_role_commit_batching_window", "Commit batching window size latency ");
    static ref P_DATA_GRV_PROXY_LATENCY: HashMap<String, GaugeVec> = LatencyStats::register("fdb_cluster_process_role_grv_proxy_latency", "GRV proxies latency");
    static ref P_DATA_GRV_PROXY_BATCHING_LATENCY: HashMap<String, GaugeVec> = LatencyStats::register("fdb_cluster_process_role_grv_proxy_batching", "GRV proxies commit batching latency");

    // Frequencies related
    static ref P_DATA_FREQ_TOTAL_QUERIES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_total_queries", "Total number of queries");
    static ref P_DATA_FREQ_FINISHED_QUERIES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_finished_queries", "Number of finished queries");
    static ref P_DATA_FREQ_LOW_PRIORITY_QUERIES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_low_priority_queries", "Number of low prio queries");
    static ref P_DATA_FREQ_BYTES_QUERIED: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_bytes_queried", "Frequency of write storage server operations in bytes");
    static ref P_DATA_FREQ_KEYS_QUERIED: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_keys_queried", "Frequency of read storage server operations in bytes");
    static ref P_DATA_FREQ_MUTATION_BYTES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_mutation_bytes", "Frequency of mutations in bytes");
    static ref P_DATA_FREQ_MUTATION: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_mutation", "Frequency of mutation");
    static ref P_DATA_FREQ_FETCHED_VERSIONS: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_fetched_versions", "Frequency of fetched versions in control plane");
    static ref P_DATA_FREQ_FETCHES_FROM_LOG: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_fetches_from_log", "Frequency of fetched data from T logs");
    static ref P_DATA_FREQ_INPUT_BYTES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_input_bytes", "Storage and Log Input Rates");
    static ref P_DATA_FREQ_DURABLE_BYTES: HashMap<String, GaugeVec> = ClusterProcessRoleFreq::register("fdb_cluster_process_role_durable_bytes", "Storage and Log input rates durable");
}

impl StaticMetric<GaugeVec> for ClusterProcessRoleFreq {
    fn register(prefix: &str, desc: &str) -> HashMap<String, GaugeVec> {
        let stat_name = &["counter", "hz", "roughness"];
        let mut metrics = HashMap::new();
        for name in stat_name {
            metrics.insert(
                name.to_string(),
                register_gauge_vec!(format!("{}_{}", prefix, name), desc, PROCESS_LABELS).unwrap(),
            );
        }
        metrics
    }
    fn set(&self, metric: &HashMap<String, GaugeVec>, labels: &[&str]) {
        let stat_name = &["counter", "hz", "roughness"];
        for name in *stat_name {
            // Safe as we know already the stat names
            let metric = metric.get(name).unwrap();
            let value: Option<f64> = match name {
                "counter" => Some(self.counter as f64),
                "hz" => Some(self.hz),
                "roughness" => Some(self.roughness),
                // Impossible case
                &_ => {
                    warn!(
                        "ClusterProcessRoleFreq::set() went through irregular case for {}",
                        name
                    );
                    None
                }
            };

            if let Some(value_f64) = value {
                metric.with_label_values(labels).set(value_f64);
            }
        }
    }
}

impl StaticMetric<GaugeVec> for LatencyStats {
    fn register(prefix: &str, desc: &str) -> HashMap<String, GaugeVec> {
        let stat_name = &[
            "count", "min", "max", "median", "mean", "p25", "p90", "p95", "p99", "p99_9",
        ];
        let mut metrics = HashMap::new();
        for name in stat_name {
            metrics.insert(
                name.to_string(),
                register_gauge_vec!(format!("{}_{}", prefix, name), desc, PROCESS_LABELS,).unwrap(),
            );
        }
        metrics
    }
    fn set(&self, metrics: &HashMap<String, GaugeVec>, labels: &[&str]) {
        let stat_name = &[
            "count", "min", "max", "median", "mean", "p25", "p90", "p95", "p99", "p99_9",
        ];
        for name in *stat_name {
            // Safe as we know already the stat names
            let metric = metrics.get(name).unwrap();
            let value: Option<f64> = match name {
                "count" => Some(self.count),
                "min" => Some(self.min),
                "max" => Some(self.max),
                "median" => Some(self.median),
                "mean" => Some(self.mean),
                "p25" => Some(self.p25),
                "p90" => Some(self.p90),
                "p95" => Some(self.p95),
                "p99" => Some(self.p99),
                "p99_9" => Some(self.p99_9),
                // Impossible case
                &_ => {
                    warn!(
                        "LatencyStats::set() went through irregular case for {}",
                        name
                    );
                    None
                }
            };

            if let Some(value_f64) = value {
                metric.with_label_values(labels).set(value_f64);
            }
        }
    }
}

impl MetricsConvertible for ClusterProcessRole {
    fn to_metrics(&self, labels: &[&str]) {
        // Kv store related
        if let Some(used_bytes) = self.kvstore_used_bytes {
            P_KVSTORE_USED_BYTES
                .with_label_values(labels)
                .set(used_bytes)
        }
        if let Some(available_bytes) = self.kvstore_available_bytes {
            P_KVSTORE_AVAILABLE_BYTES
                .with_label_values(labels)
                .set(available_bytes)
        }
        if let Some(free_bytes) = self.kvstore_free_bytes {
            P_KVSTORE_FREE_BYTES
                .with_label_values(labels)
                .set(free_bytes)
        }
        // Queue related
        if let Some(queue_max) = self.query_queue_max {
            P_QUERY_QUEUE_MAX.with_label_values(labels).set(queue_max);
        }
        if let Some(used_bytes) = self.queue_disk_used_bytes {
            P_QUEUE_DISK_USED_BYTES
                .with_label_values(labels)
                .set(used_bytes);
        }
        if let Some(available_bytes) = self.queue_disk_available_bytes {
            P_QUEUE_DISK_AVAILABLE_BYTES
                .with_label_values(labels)
                .set(available_bytes);
        }
        if let Some(free_bytes) = self.queue_disk_free_bytes {
            P_QUEUE_DISK_FREE_BYTES
                .with_label_values(labels)
                .set(free_bytes);
        }
        if let Some(total_bytes) = self.queue_disk_total_bytes {
            P_QUEUE_DISK_TOTAL_BYTES
                .with_label_values(labels)
                .set(total_bytes);
        }

        // Lag related
        if let Some(data_lag) = &self.data_lag {
            P_DATA_LAG_SECONDS
                .with_label_values(labels)
                .set(data_lag.seconds);
        }
        if let Some(durable_lag) = &self.durability_lag {
            P_DATA_DURABLE_LAG_SECONDS
                .with_label_values(labels)
                .set(durable_lag.seconds);
        }

        // Roles global latency stats (storage, commit_proxy...)
        self.read_latency_statistics
            .and_set_with_labels(&P_DATA_READ_LATENCY, labels);
        self.commit_latency_statistics
            .and_set_with_labels(&P_DATA_COMMIT_LATENCY, labels);

        // grv_proxy roles latency stats
        if let Some(default_latencies) = &self.grv_latency_statistics {
            default_latencies
                .default
                .and_set_with_labels(&P_DATA_GRV_PROXY_LATENCY, labels);
            default_latencies
                .batch
                .and_set_with_labels(&P_DATA_GRV_PROXY_BATCHING_LATENCY, labels);
        }
        self.commit_batching_window_size
            .and_set_with_labels(&P_DATA_COMMIT_BATCHING_WINDOW_SIZE, labels);

        // Frequencies related
        self.total_queries
            .and_set_with_labels(&P_DATA_FREQ_TOTAL_QUERIES, labels);
        self.finished_queries
            .and_set_with_labels(&P_DATA_FREQ_FINISHED_QUERIES, labels);
        self.low_priority_queries
            .and_set_with_labels(&P_DATA_FREQ_LOW_PRIORITY_QUERIES, labels);
        self.bytes_queried
            .and_set_with_labels(&P_DATA_FREQ_BYTES_QUERIED, labels);
        self.keys_queried
            .and_set_with_labels(&P_DATA_FREQ_KEYS_QUERIED, labels);
        self.mutation_bytes
            .and_set_with_labels(&P_DATA_FREQ_MUTATION_BYTES, labels);
        self.mutations
            .and_set_with_labels(&P_DATA_FREQ_MUTATION, labels);
        self.fetched_versions
            .and_set_with_labels(&P_DATA_FREQ_FETCHED_VERSIONS, labels);
        self.fetches_from_logs
            .and_set_with_labels(&P_DATA_FREQ_FETCHES_FROM_LOG, labels);
        self.input_bytes
            .and_set_with_labels(&P_DATA_FREQ_INPUT_BYTES, labels);
        self.durable_bytes
            .and_set_with_labels(&P_DATA_FREQ_DURABLE_BYTES, labels);
    }
}
