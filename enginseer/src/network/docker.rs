use log::debug;

use crate::{network::configs::storage::NetworkConfig, workload::errors::VMError};

#[derive(Debug)]
pub struct DockerNetwork {
    pub name: String,
}

pub fn docker_network_from_config(network: &NetworkConfig) -> Result<DockerNetwork, VMError> {
    match network {
        NetworkConfig::Bridge { common, data } => Ok(DockerNetwork {
            name: common.name.clone(),
        }),
        NetworkConfig::Nat { common, data } => Ok(DockerNetwork {
            name: common.name.clone(),
        }),
        NetworkConfig::MacVlan { common, data } => Ok(DockerNetwork {
            name: "none".to_string(),
        }),
        NetworkConfig::None {} => {
            debug!("network is none");
            todo!();
        }
    }
}
