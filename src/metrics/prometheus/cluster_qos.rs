use lazy_static::lazy_static;
use prometheus::{register_gauge, register_int_gauge, Gauge, IntGauge};
use std::collections::HashMap;
use tracing::warn;

use crate::{
    metrics::MetricsConvertible,
    status_models::{cluster_process_role::DataLag, cluster_qos::ClusterQos},
};

use super::{AndSet, StaticMetric};

lazy_static! {
    static ref P_LIMITING_QUEUE_STORAGE_SERVER_BYTES: IntGauge = register_int_gauge!(
        "fdb_qos_limiting_queue_storage_server_bytes",
        "Queue of the storage server limiting the system"
    )
    .unwrap();
    static ref P_LIMITING_DATA_STORAGE: HashMap<String, Gauge> = DataLag::register(
        "fdb_qos_limiting_data_lag_storage_server",
        "Lag of the limiting storage server"
    );
    static ref P_LIMITING_DURABILITY_LAG_STORAGE: HashMap<String, Gauge> = DataLag::register(
        "fdb_qos_limiting_durability_lag_storage_server",
        "Durability lag of the limiting storage server"
    );
    static ref P_WORST_DATA_LAG_STORAGE_SERVER: HashMap<String, Gauge> = DataLag::register(
        "fdb_qos_worst_data_lag_storage_server",
        "Storage server with the worst queue"
    );
    static ref P_WORST_DURABILITY_LAG_STORAGE_SERVER: HashMap<String, Gauge> = DataLag::register(
        "fdb_qos_worst_durability_lag_storage_server",
        "Storage server with the worst durability queue"
    );
    static ref P_WORST_QUEUE_BYTES_LOG_SERVER: IntGauge = register_int_gauge!(
        "fdb_qos_worst_queue_log_server_bytes",
        "Worst queue of log server in bytes"
    )
    .unwrap();
    static ref P_WORST_QUEUE_BYTES_STORAGE_SERVER: IntGauge = register_int_gauge!(
        "fdb_qos_worst_queue_storage_server_bytes",
        "Worst queue of storage server",
    )
    .unwrap();
    static ref P_PERFORMANCE_LIMITED_BY_REASON: IntGauge = register_int_gauge!(
        "fdb_qos_performance_limited_by_reason",
        "Reason of the system being limited"
    )
    .unwrap();
    static ref P_TRANSACTIONS_PER_SERCOND_LIMIT: Gauge = register_gauge!(
        "fdb_qos_transactions_per_second_limit",
        "Number of transactions the cluster allows per second"
    )
    .unwrap();
    static ref P_BATCH_TRANSACTIONS_PER_SECOND_LIMIT: Gauge = register_gauge!(
        "fdb_qos_batch_transactions_per_second_limit",
        "Number of batch transactions the cluster allows per second"
    )
    .unwrap();
}

impl MetricsConvertible for ClusterQos {
    fn to_metrics(&self, _: &[&str]) {
        P_LIMITING_QUEUE_STORAGE_SERVER_BYTES.set(self.limiting_queue_bytes_storage_server);
        self.limiting_data_lag_storage_server
            .and_set(&P_LIMITING_DATA_STORAGE);
        self.limiting_durability_lag_storage_server
            .and_set(&P_LIMITING_DURABILITY_LAG_STORAGE);
        self.worst_data_lag_storage_server
            .and_set(&P_WORST_DATA_LAG_STORAGE_SERVER);
        self.worst_durability_lag_storage_server
            .and_set(&P_WORST_DURABILITY_LAG_STORAGE_SERVER);

        P_WORST_QUEUE_BYTES_LOG_SERVER.set(self.worst_queue_bytes_log_server);
        P_WORST_QUEUE_BYTES_STORAGE_SERVER.set(self.worst_queue_bytes_storage_server);

        P_PERFORMANCE_LIMITED_BY_REASON.set(self.performance_limited_by.reason_id);

        P_BATCH_TRANSACTIONS_PER_SECOND_LIMIT.set(self.batch_transactions_per_second_limit);
        P_TRANSACTIONS_PER_SERCOND_LIMIT.set(self.transactions_per_second_limit);
    }
}

impl StaticMetric<Gauge> for DataLag {
    fn register(prefix: &str, desc: &str) -> HashMap<String, Gauge> {
        let stat_name = &["versions", "seconds"];
        let mut metrics = HashMap::new();
        for name in stat_name {
            metrics.insert(
                name.to_string(),
                register_gauge!(format!("{}_{}", prefix, name), desc,).unwrap(),
            );
        }
        metrics
    }
    fn set(&self, metrics: &HashMap<String, Gauge>, _: &[&str]) {
        let stat_name = &["versions", "seconds"];
        for name in *stat_name {
            // Safe as we know already the stat names
            let metric = metrics.get(name).unwrap();
            let value: f64 = match name {
                "versions" => self.versions as f64,
                "seconds" => self.seconds,
                // Impossible case
                &_ => {
                    warn!("DataLag::set() went through irregular case for {}", name);
                    -1.0
                }
            };
            metric.set(value);
        }
    }
}
