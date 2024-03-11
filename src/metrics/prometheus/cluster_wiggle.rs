use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};

use crate::{
    metrics::MetricsConvertible,
    status_models::cluster_wiggle::{ClusterStoragePrimaryWiggle, ClusterStorageWiggle},
};

const P_PREFIX: &str = "fdb_cluster_wiggle";

lazy_static! {
    /// [ClusterStorageWiggle] related
    static ref P_CLUSTER_WIGGLE_SERVER_COUNT: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "servers_count"),
        "Current number of storage servers being wiggle"
    )
    .unwrap();

    /// [ClusterStoragePrimaryWiggle] related
    static ref P_CLUSTER_WIGGLE_FINISHED_ROUNDS_COUNT: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "finished_rounds_count"),
        "Number of finished rounds",
    ).unwrap();

    static ref P_CLUSTER_WIGGLE_FINISHED_COUNT: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "finished_count"),
        "Number of finished wiggle"
    ).unwrap();

    static ref P_CLUSTER_WIGGLE_SMOOTHED_ROUND_SECONDS: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "smoothed_round_seconds"),
        "Seconds elapsed in the current round"
    ).unwrap();
    static ref P_CLUSTER_WIGGLE_SMOOTHED_SECONDS: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "smoothed_seconds"),
        "Seconds elapsed in the current wiggle"
    ).unwrap();
    static ref P_CLUSTER_WIGGLE_LAST_ROUND_FINISH: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "last_round_finish"),
        "Timestamp of the last fully finished round"
    ).unwrap();
    static ref P_CLUSTER_WIGGLE_LAST_FINISH: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "last_finish"),
        "Timestamp of the last fully finished wiggle"
    ).unwrap();
    static ref P_CLUSTER_WIGGLE_LAST_ROUND_START: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "last_round_start"),
        "Timestamp of the start of last round"
    ).unwrap();
    static ref P_CLUSTER_WIGGLE_LAST_START: IntGauge = register_int_gauge!(
        format!("{}_{}", P_PREFIX, "last_start"),
        "Timestamp of the start of last wiggle"
    ).unwrap();
}

impl MetricsConvertible for ClusterStorageWiggle {
    fn to_metrics(&self, _: &[&str]) {
        P_CLUSTER_WIGGLE_SERVER_COUNT.set(self.wiggle_server_addresses.len() as i64);
        if let Some(primary) = &self.primary {
            primary.to_metrics(&[]);
        }
    }
}

impl MetricsConvertible for ClusterStoragePrimaryWiggle {
    fn to_metrics(&self, _: &[&str]) {
        P_CLUSTER_WIGGLE_FINISHED_ROUNDS_COUNT.set(self.finished_round as i64);
        P_CLUSTER_WIGGLE_FINISHED_COUNT.set(self.finished_wiggle as i64);

        P_CLUSTER_WIGGLE_SMOOTHED_ROUND_SECONDS.set(self.smoothed_round_seconds.floor() as i64);
        P_CLUSTER_WIGGLE_SMOOTHED_SECONDS.set(self.smoothed_wiggle_seconds.floor() as i64);

        P_CLUSTER_WIGGLE_LAST_ROUND_FINISH.set(self.last_round_finish_timestamp.floor() as i64);
        P_CLUSTER_WIGGLE_LAST_FINISH.set(self.last_wiggle_finish_timestamp.floor() as i64);

        P_CLUSTER_WIGGLE_LAST_ROUND_START.set(self.last_round_start_timestamp.floor() as i64);
        P_CLUSTER_WIGGLE_LAST_START.set(self.last_wiggle_start_timestamp.floor() as i64);
    }
}
