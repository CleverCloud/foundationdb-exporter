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

/// jq: .cluster.data.state
#[derive(Deserialize)]
pub struct ClusterDataState {
    pub healthy: bool,
    pub description: Option<String>,
    pub min_replicas_remaining: i64,
    // available values:
    // "initializing",
    // "missing_data",
    // "healing",
    // "optimizing_team_collections",
    // "healthy_populating_region",
    // "healthy_repartitioning",
    // "healthy_removing_server",
    // "healthy_rebalancing",
    // "healthy"
    pub name: String,
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
