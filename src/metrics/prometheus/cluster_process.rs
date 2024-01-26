use super::PROCESS_LABELS;
use crate::{metrics::MetricsConvertible, status_models::cluster_process::ClusterProcess};
use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};

lazy_static! {
    static ref P_PROCESS_EXCLUDED: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_excluded",
        "Process is being excluded by the cluster",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_CPU_USAGE: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_cpu_usage",
        "Current usage of CPU (between 0 and 1)",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_UPTIME: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_uptime",
        "Uptime of the process",
        PROCESS_LABELS,
    )
    .unwrap();
    static ref P_PROCESS_RUN_LOOP_BUSY: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_busy",
        "Busy of the process (value between 0.0 and 1.1)",
        PROCESS_LABELS,
    )
    .unwrap();
}

impl MetricsConvertible for ClusterProcess {
    fn to_metrics(&self, labels: &[&str]) {
        if let Some(uptime) = self.uptime_seconds {
            P_PROCESS_UPTIME.with_label_values(labels).set(uptime);
        }

        if let Some(run_loop_busy) = self.run_loop_busy {
            P_PROCESS_RUN_LOOP_BUSY
                .with_label_values(labels)
                .set(run_loop_busy);
        }

        if let Some(excluded) = self.excluded {
            P_PROCESS_EXCLUDED
                .with_label_values(labels)
                .set(excluded as i64);
        }

        if let Some(cpu) = &self.cpu {
            P_PROCESS_CPU_USAGE
                .with_label_values(labels)
                .set(cpu.usage_cores);
        }

        if let Some(disk) = &self.disk {
            disk.to_metrics(labels);
        }

        if let Some(network) = &self.network {
            network.to_metrics(labels);
        }

        if let Some(memory) = &self.memory {
            memory.to_metrics(labels);
        }

        for role in &self.roles {
            role.to_metrics(labels);
        }
    }
}
