use crate::fetcher::Error as FetcherError;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, register_int_counter_vec, IntCounter, IntCounterVec};

use super::MetricsConvertible;

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
            FetcherError::Parsing(e) => {
                let path = e.path().to_string();
                P_FDB_EXPORTER_PARSING_ERROR.inc();
            }
        };
    }
}
