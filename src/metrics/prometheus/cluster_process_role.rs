use std::collections::HashMap;

use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};

use crate::{
    metrics::MetricsConvertible,
    status_models::cluster_process_role::{ClusterProcessRole, LatencyStats},
};

lazy_static! {
    // Queue related
    static ref P_QUEUE_DISK_USED_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_used_bytes",
        "Used bytes in the queue of a process",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    static ref P_QUEUE_DISK_AVAILABLE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_available_bytes",
        "Available bytes in the queue of a process",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    static ref P_QUEUE_DISK_FREE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_free_bytes",
        "Free bytes in the queue of a process",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    static ref P_QUEUE_DISK_TOTAL_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_queue_disk_total_bytes",
        "Total bytes in the queue of a process",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    // Lag related
    static ref P_DATA_LAG_SECONDS: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_data_lag_seconds",
        "Lag in seconds of the process role",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    static ref P_DATA_DURABLE_LAG_SECONDS: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_role_durable_lag_seconds",
        "Lag in seconds of data being durable of the process role",
        &["machine_id", "process_id", "class_type"],
    ).unwrap();

    static ref P_DATA_READ_LATENCY: HashMap<String, GaugeVec> = generate_latency_stats("fdb_cluster_process_role_read_latency", "Latency of read");
    static ref P_DATA_COMMIT_LATENCY: HashMap<String, GaugeVec> = generate_latency_stats("fdb_cluster_process_role_commit_latency", "Latency to commit");
    static ref P_DATA_COMMIT_BATCHING_WINDOW_SIZE: HashMap<String, GaugeVec> = generate_latency_stats("fdb_cluster_process_role_commit_batching_window", "Commit batching window size latency ");
}

fn generate_latency_stats(prefix: &str, desc: &str) -> HashMap<String, GaugeVec> {
    let stat_name = &[
        "count", "min", "max", "median", "mean", "p25", "p90", "p95", "p99", "p99_9",
    ];
    let mut metrics = HashMap::new();
    for name in stat_name {
        metrics.insert(
            name.to_string(),
            register_gauge_vec!(
                format!("{}_{}", prefix, name),
                desc,
                &["machine_id", "process_id", "class_type"],
            )
            .unwrap(),
        );
    }
    metrics
}

fn set_latency_stats(metrics: &HashMap<String, GaugeVec>, labels: &[&str], stats: &LatencyStats) {
    let stat_name = &[
        "count", "min", "max", "median", "mean", "p25", "p90", "p95", "p99", "p99_9",
    ];
    for name in *stat_name {
        // Safe as we know already the stat names
        let metric = metrics.get(name).unwrap();
        let value: f64 = match name {
            "count" => stats.count,
            "min" => stats.min,
            "max" => stats.max,
            "median" => stats.median,
            "mean" => stats.mean,
            "p25" => stats.p25,
            "p90" => stats.p90,
            "p95" => stats.p95,
            "p99" => stats.p99,
            "p99_9" => stats.p99_9,
            // Impossible case
            &_ => -1.0,
        };
        metric.with_label_values(labels).set(value);
    }
}

impl MetricsConvertible for ClusterProcessRole {
    fn to_metrics(&self, labels: &[&str]) {
        // Queue related
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

        // Latency stats
        if let Some(read_latency) = &self.read_latency_statistics {
            set_latency_stats(&P_DATA_READ_LATENCY, labels, read_latency);
        }
        if let Some(commit_latency) = &self.commit_latency_statistics {
            set_latency_stats(&P_DATA_COMMIT_LATENCY, labels, commit_latency);
        }
        if let Some(commit_batching_window) = &self.commit_batching_window_size {
            set_latency_stats(
                &P_DATA_COMMIT_BATCHING_WINDOW_SIZE,
                labels,
                commit_batching_window,
            );
        }
    }
}
