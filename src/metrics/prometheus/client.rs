use crate::metrics::MetricsConvertible;
use crate::status_models::client::ClientStatus;
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, register_int_gauge_vec, IntGauge, IntGaugeVec};

lazy_static! {
    static ref P_CLIENT_TIMESTAMP: IntGauge =
        register_int_gauge!("fdb_client_timestamp", "Client timestamp when last fetched").unwrap();
    static ref P_CLIENT_COORDINATORS_COUNT: IntGauge = register_int_gauge!(
        "fdb_client_coordinators_count",
        "Number of coordinators registered in client fdb.cluster"
    )
    .unwrap();
    static ref P_CLIENT_COORDINATOR_REACHABLE: IntGaugeVec = register_int_gauge_vec!(
        "fdb_client_coordinator_reachable",
        "Whether the coordinator is reachable",
        &["address"],
    )
    .unwrap();
    static ref P_CLIENT_QUORUM_REACHABLE: IntGauge = register_int_gauge!(
        "fdb_client_quorum_reachable",
        "The quorum of coordinators is reachable"
    )
    .unwrap();
    static ref P_CLIENT_MESSAGES_COUNT: IntGauge = register_int_gauge!(
        "fdb_client_messages_count",
        "Number of messages available when fetching status",
    )
    .unwrap();
    static ref P_CLIENT_DATABASE_AVAILABLE: IntGauge = register_int_gauge!(
        "fdb_database_available",
        "Database can receive request (0=unavailable)"
    )
    .unwrap();
    static ref P_CLIENT_DATABASE_HEALTHY: IntGauge =
        register_int_gauge!("fdb_database_healthy", "Database healthiness (0=unhealthy)").unwrap();
}

impl MetricsConvertible for ClientStatus {
    fn to_metrics(&self, _: &[&str]) {
        if let Some(timestamp) = self.timestamp {
            P_CLIENT_TIMESTAMP.set(timestamp)
        }

        let coordinators_count = self.coordinators.coordinators.len() as i64;
        P_CLIENT_COORDINATORS_COUNT.set(coordinators_count);

        for coordinator in &self.coordinators.coordinators {
            let addr = coordinator.address.to_string();
            P_CLIENT_COORDINATOR_REACHABLE
                .with_label_values(&[(addr.as_str())])
                .set(coordinator.reachable as i64);
        }

        P_CLIENT_QUORUM_REACHABLE.set(self.coordinators.quorum_reachable as i64);

        P_CLIENT_MESSAGES_COUNT.set(self.messages.len() as i64);

        P_CLIENT_DATABASE_HEALTHY.set(self.database_status.healthy as i64);
        P_CLIENT_DATABASE_AVAILABLE.set(self.database_status.available as i64);
    }
}
