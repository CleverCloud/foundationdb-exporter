use serde::Deserialize;

pub mod client;
pub mod cluster;
pub mod cluster_data;
pub mod cluster_machine;
pub mod cluster_process;
pub mod cluster_process_disk;
pub mod cluster_process_memory;
pub mod cluster_process_network;

#[derive(Deserialize)]
pub struct Status {
    pub client: client::ClientStatus,
    pub cluster: Option<cluster::ClusterStatus>,
}
