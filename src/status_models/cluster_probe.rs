use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClusterLatencyProbe {
    pub commit_seconds: Option<f64>,
    pub immediate_priority_start_seconds: Option<f64>,
    pub read_seconds: Option<f64>,
    pub transaction_start_seconds: Option<f64>,
}
