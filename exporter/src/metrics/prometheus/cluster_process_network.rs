use crate::{
    metrics::MetricsConvertible, status_models::cluster_process_network::ClusterProcessNetwork,
};
use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, GaugeVec};

lazy_static! {
    static ref P_PROCESS_NETWORK_CONN_ERRORS: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_network_connection_errors_freq",
        "Frequency of connection errors",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_NETWORK_CONN_CLOSED: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_network_connections_closed",
        "Frequency of connection closed",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_NETWORK_CONN_ESTABLISHED: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_network_connections_established",
        "Frequency of connection established",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_NETWORK_MEGABITS_RECEIVED: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_network_received_megabits",
        "Megabits received on network",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_NETWORK_MEGABITS_SENT: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_network_sent_megabits",
        "Megabits sent on network",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
}

impl MetricsConvertible for ClusterProcessNetwork {
    fn to_metrics(&self, labels: &[&str]) {
        P_PROCESS_NETWORK_CONN_ERRORS
            .with_label_values(labels)
            .set(self.connection_errors.hz);
        P_PROCESS_NETWORK_CONN_CLOSED
            .with_label_values(labels)
            .set(self.connections_closed.hz);
        P_PROCESS_NETWORK_CONN_ESTABLISHED
            .with_label_values(labels)
            .set(self.connections_established.into());
        P_PROCESS_NETWORK_MEGABITS_RECEIVED
            .with_label_values(labels)
            .set(self.megabits_received.into());
        P_PROCESS_NETWORK_MEGABITS_SENT
            .with_label_values(labels)
            .set(self.megabits_sent.into());
    }
}
