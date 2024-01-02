use serde::Deserialize;

/// jq: .cluster.data
#[derive(Deserialize)]
pub struct ClusterData {
    pub average_partition_size_bytes: i64,
    pub least_operating_space_bytes_log_server: i64,
    pub least_operating_space_bytes_storage_server: i64,
    pub moving_data: Option<ClusterDataMoving>,
    pub partitions_count: i64,
    pub total_disk_used_bytes: i64,
    pub total_kv_size_bytes: i64,
    pub state: ClusterDataState,
}

// jq: .cluster.data.state.name
#[derive(Deserialize, Copy, Clone)]
pub enum ClusterDataStateName {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "missing_data")]
    MissingData,
    #[serde(rename = "healing")]
    Healing,
    #[serde(rename = "optimizing_team_collections")]
    OptimizingTeamCollections,
    #[serde(rename = "healthy_populating_region")]
    HealthyPopulatingRegion,
    #[serde(rename = "healthy_repartitioning")]
    HealthyRepartitioning,
    #[serde(rename = "healthy_removing_server")]
    HealthyRemovingServer,
    #[serde(rename = "healthy_rebalancing")]
    HealthyRebalacing,
    #[serde(rename = "healthy")]
    Healthy,
}

/// jq: .cluster.data.state
#[derive(Deserialize)]
pub struct ClusterDataState {
    pub healthy: bool,
    pub description: Option<String>,
    pub min_replicas_remaining: i64,
    pub name: ClusterDataStateName,
}

/// jq: .cluster.data.moving_data
#[derive(Deserialize)]
pub struct ClusterDataMoving {
    pub highest_priority: i64,
    pub in_flight_bytes: i64,
    pub in_queue_bytes: i64,
    // reset whenever data distributor is re-recruited
    pub total_written_bytes: i64,
}
