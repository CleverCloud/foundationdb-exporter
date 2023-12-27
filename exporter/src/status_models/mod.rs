use serde::Deserialize;

pub mod client;
pub mod cluster;
pub mod cluster_machine;

#[derive(Deserialize)]
pub struct Status {
    pub client: client::ClientStatus,
    pub cluster: Option<cluster::ClusterStatus>,
}
