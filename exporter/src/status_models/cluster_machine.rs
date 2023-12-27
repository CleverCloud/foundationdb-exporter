use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct MachineId(pub String);

#[derive(Deserialize)]
pub struct Frequency {
    pub hz: f64,
}

/// jq: .cluster.machines[]
#[derive(Deserialize)]
pub struct ClusterMachine {
    pub machine_id: MachineId,
    pub address: String,
    pub excluded: bool,
    pub datacenter_id: Option<String>,
    pub memory: ClusterMachineMemory,
    pub contributing_workers: u32,
    pub network: ClusterMachineNetwork,
}

/// jq: .cluster.machines[].memory
#[derive(Deserialize)]
pub struct ClusterMachineMemory {
    pub free_bytes: u64,
    pub committed_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Deserialize)]
pub struct ClusterMachineNetwork {
    pub megabits_sent: Frequency,
    pub megabits_received: Frequency,
    pub tcp_segments_retransmitted: Frequency,
}
