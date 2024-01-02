use crate::status_models::cluster::ClusterStatus;
use crate::{metrics::MetricsConvertible, status_models::cluster_process::ClusterClassType};
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};

lazy_static! {
    static ref P_CLUSTER_MACHINES_COUNT: IntGauge = register_int_gauge!(
        "fdb_cluster_machines_count",
        "Number of machines available in the cluster"
    )
    .unwrap();
    static ref P_CLUSTER_GENERATION_COUNT: IntGauge =
        register_int_gauge!("fdb_cluster_generation_count", "Number of generations").unwrap();
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

        for (process_id, process) in &self.processes {
            let machine_id = &process.machine_id;
            let class_type = process
                .class_type
                .as_ref()
                .unwrap_or(&ClusterClassType::Unset)
                .to_string();
            let labels = [
                machine_id.0.as_str(),
                process_id.0.as_str(),
                class_type.as_str(),
            ];
            process.to_metrics(&labels);
        }

        if let Some(latency_probe) = &self.latency_probe {
            latency_probe.to_metrics(&[]);
        }

        P_CLUSTER_GENERATION_COUNT.set(self.generation);
    }
}
