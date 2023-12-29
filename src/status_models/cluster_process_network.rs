use super::cluster_machine::Frequency;
use serde::Deserialize;

/// jq: .cluster.processes[].network
#[derive(Deserialize)]
pub struct ClusterProcessNetwork {
    pub connection_errors: Frequency,
    pub connections_closed: Frequency,
    pub connections_established: Frequency,
    pub current_connections: i64,
    pub megabits_received: Frequency,
    pub megabits_sent: Frequency,
    pub tls_policy_failures: Frequency,
}
