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
    let labels = vec![];
    new_status.client.to_metrics(&labels);
    if let Some(cluster) = new_status.cluster {
        cluster.to_metrics(&labels);
    }
}
