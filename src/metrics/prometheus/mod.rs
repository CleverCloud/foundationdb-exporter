use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use std::collections::HashMap;

use super::MetricsConvertible;
use crate::fetcher::Error as FetcherError;

pub mod client;
pub mod cluster;
pub mod cluster_data;
pub mod cluster_machines;
pub mod cluster_probe;
pub mod cluster_process;
pub mod cluster_process_disk;
pub mod cluster_process_memory;
pub mod cluster_process_network;
pub mod cluster_process_role;
pub mod cluster_qos;

#[macro_export]
macro_rules! register_data_lag {
    ($prefix:expr, $desc:expr, $labels: expr) => {{
        let mut metrics: HashMap<String, GaugeVec> = HashMap::new();
        for key_name in ["versions", "seconds"] {
            let metric = register_gauge_vec!(
                format!("{}_{}", $prefix, key_name),
                format!("{} ({})", $desc, key_name),
                $labels
            )
            .unwrap();

            metrics.insert(String::from(key_name), metric);
        }
        metrics
    }};

    ($prefix:expr, $desc:expr) => {
        register_data_lag!($prefix, $desc, &[])
    };
}

#[macro_export]
macro_rules! set_data_lag {
    ($key:ident, $data_lag:ident, $labels:expr) => {
        $key.get("versions")
            .unwrap()
            .with_label_values($labels)
            .set($data_lag.versions as f64);
        $key.get("seconds")
            .unwrap()
            .with_label_values($labels)
            .set($data_lag.seconds);
    };
    ($key:ident, $data_lag:ident) => {
        set_data_lag!($key, $data_lag, &[]);
    };
}

lazy_static! {
    static ref P_FDB_EXPORTER_PARSING_ERROR: IntCounter = register_int_counter! {
        "fdb_exporter_parsing_error_count",
        "Number of parsing errors encountered",
    }
    .unwrap();
    static ref P_FDB_EXPORTER_CMD_ERROR: IntCounter = register_int_counter!(
        "fdb_exporter_cmd_error_count",
        "Number of error running the command line"
    )
    .unwrap();
}

impl MetricsConvertible for FetcherError {
    fn to_metrics(&self, _: &[&str]) {
        match self {
            FetcherError::Cmd(_) => P_FDB_EXPORTER_CMD_ERROR.inc(),
            FetcherError::Parsing(_) => P_FDB_EXPORTER_PARSING_ERROR.inc(),
        };
    }
}

/// Implements methods that should be used to register more than one metric on a type
/// [StaticMetric::register] should only be used in lazy_static to generate metrics
/// [StaticMetric::set] should be used in methods to apply new values
pub trait StaticMetric<T> {
    /// Generate a HashMap matching type T with all necessary values to be exposed
    fn register(prefix: &str, desc: &str) -> HashMap<String, T>;
    /// Apply T metrics on the HashMap
    /// [StaticMetric::register] MUST have been used before calling set
    fn set(&self, metric: &HashMap<String, T>, labels: &[&str]);
}

pub trait AndSet<T> {
    fn and_set(&self, metric: &HashMap<String, T>);
    fn and_set_with_labels(&self, metric: &HashMap<String, T>, labels: &[&str]);
}

impl<M, T> AndSet<T> for Option<M>
where
    M: StaticMetric<T>,
{
    fn and_set(&self, metric: &HashMap<String, T>) {
        self.and_set_with_labels(metric, &[]);
    }

    fn and_set_with_labels(&self, metric: &HashMap<String, T>, labels: &[&str]) {
        if let Some(v) = self {
            v.set(metric, labels);
        }
    }
}
