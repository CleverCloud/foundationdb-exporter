use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge, GaugeVec, IntGauge};
use std::collections::HashMap;

use crate::{
    metrics::MetricsConvertible, register_data_lag, set_data_lag,
    status_models::cluster_qos::ClusterQos,
};

lazy_static! {
    static ref P_LIMITING_DATA_STORAGE: HashMap<String, GaugeVec> = register_data_lag!(
        "fdb_qos_limiting_data_lag_storage_server",
        "Lag of the limiting storage server"
    );
    static ref P_LIMITING_DURABILITY_LAG_STORAGE: HashMap<String, GaugeVec> = register_data_lag!(
        "fdb_qos_limiting_durability_lag_storage_server",
        "Durability lag of the limiting storage server"
    );
    static ref P_WORST_DATA_LAG_STORAGE_SERVER: HashMap<String, GaugeVec> = register_data_lag!(
        "fdb_qos_worst_data_lag_storage_server",
        "Storage server with the worst queue"
    );
    static ref P_WORST_DURABILITY_LAG_STORAGE_SERVER: HashMap<String, GaugeVec> = register_data_lag!(
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
}

impl MetricsConvertible for ClusterQos {
    fn to_metrics(&self, _: &[&str]) {
        if let Some(limit_data_store) = &self.limiting_data_lag_storage_server {
            set_data_lag!(P_LIMITING_DATA_STORAGE, limit_data_store);
        }

        if let Some(limit_durability_store) = &self.limiting_durability_lag_storage_server {
            set_data_lag!(P_LIMITING_DURABILITY_LAG_STORAGE, limit_durability_store);
        }

        if let Some(worst_data_store) = &self.worst_data_lag_storage_server {
            set_data_lag!(P_WORST_DATA_LAG_STORAGE_SERVER, worst_data_store);
        }

        if let Some(worst_durability_store) = &self.worst_durability_lag_storage_server {
            set_data_lag!(
                P_WORST_DURABILITY_LAG_STORAGE_SERVER,
                worst_durability_store
            );
        }

        P_WORST_QUEUE_BYTES_LOG_SERVER.set(self.worst_queue_bytes_log_server);
        P_WORST_QUEUE_BYTES_STORAGE_SERVER.set(self.worst_queue_bytes_storage_server);

        P_PERFORMANCE_LIMITED_BY_REASON.set(self.performance_limited_by.reason_id);
    }
}
