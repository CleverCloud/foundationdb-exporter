use lazy_static::lazy_static;
use prometheus::{register_int_counter, Gauge, IntCounter, IntGauge};
use std::collections::HashMap;

use super::MetricsConvertible;
use crate::fetcher::Error as FetcherError;

pub mod client;
pub mod cluster;
pub mod cluster_backup;
pub mod cluster_data;
pub mod cluster_machines;
pub mod cluster_probe;
pub mod cluster_process;
pub mod cluster_process_disk;
pub mod cluster_process_memory;
pub mod cluster_process_network;
pub mod cluster_process_role;
pub mod cluster_qos;

pub const PROCESS_LABELS: &[&str] = &["machine_id", "process_id", "class_type", "address"];

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
    /// Borrow [Self] to update a HashMap of metrics
    fn and_set(&self, metric: &HashMap<String, T>);
    /// Borrow [Self] to update a HashMap of metrics where each
    /// metric will have specific labels
    fn and_set_with_labels(&self, metric: &HashMap<String, T>, labels: &[&str]);
}

pub trait AndSetSingle<T> {
    /// Borrow [Self] and update metric [T]
    fn and_set(&self, metric: &T);
}

impl AndSetSingle<IntGauge> for Option<i64> {
    fn and_set(&self, metric: &IntGauge) {
        if let Some(item) = self {
            metric.set(*item);
        }
    }
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
