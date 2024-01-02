use crate::status_models::cluster_data::ClusterData;
use crate::status_models::cluster_machine::{ClusterMachine, MachineId};
use serde::Deserialize;
use std::collections::HashMap;

use super::cluster_probe::ClusterLatencyProbe;
use super::cluster_process::{ClusterProcess, ProcessId};

/// jq: .cluster
#[derive(Deserialize)]
pub struct ClusterStatus {
    #[serde(default)]
    pub database_available: bool,
    #[serde(default)]
    pub machines: HashMap<MachineId, ClusterMachine>,
    pub data: ClusterData,
    pub processes: HashMap<ProcessId, ClusterProcess>,
    pub latency_probe: Option<ClusterLatencyProbe>,
}
