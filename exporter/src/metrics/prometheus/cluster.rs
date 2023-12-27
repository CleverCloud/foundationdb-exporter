use crate::metrics::MetricsConvertible;
use crate::status_models::cluster::ClusterStatus;
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};

lazy_static! {
    static ref P_CLUSTER_MACHINES_COUNT: IntGauge = register_int_gauge!(
        "fdb_cluster_machines_count",
        "Number of machines available in the cluster"
    )
    .unwrap();
}

impl MetricsConvertible for ClusterStatus {
    fn to_metrics(&self, _: &[&str]) {
        P_CLUSTER_MACHINES_COUNT.set(self.machines.len() as i64);

        for (machine_id, machine) in &self.machines {
            let datacenter_id = machine
                .datacenter_id
                .clone()
                .unwrap_or(String::from("default"));
            let labels = [machine_id.0.as_str(), datacenter_id.as_str()];
            machine.to_metrics(&labels);
        }

        self.data.to_metrics(&[]);
    }
}
