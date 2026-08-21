use crate::status_models::Status;

mod prometheus;

/// Must be implemented on metrics which are updating exported metrics,
/// this trait is useful to allow usage of feature flags to have various
/// exporters (Prometheus, warp10...)
pub trait MetricsConvertible {
    fn to_metrics(&self, labels: &[&str]);
}

/// Use the status to update metrics with new status given
pub fn process_metrics(new_status: Status) {
    // `status json` is a snapshot: drop label sets from the previous poll so that
    // processes, machines, coordinators, roles and backup tags which have left the
    // cluster stop being exported. See [`self::prometheus::reset_dynamic_metrics`].
    self::prometheus::reset_dynamic_metrics();

    let labels = vec![];
    new_status.client.to_metrics(&labels);
    if let Some(cluster) = new_status.cluster {
        cluster.to_metrics(&labels);
    }
}
