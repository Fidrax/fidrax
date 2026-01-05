use log::{debug, info};

use crate::network::{
    configs::storage::{NetworkConfig, NetworkUnit},
    errors::NetworkError,
    linux_bridge::LinuxBridge,
    natgateway::NatGateway,
    store::NetworkStore,
};

#[derive(Debug, Clone)]
pub struct NetworkManager {
    pub store: NetworkStore,
}

impl NetworkManager {
    pub fn new(store: NetworkStore) -> Self {
        Self { store }
    }

    pub async fn create(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        info!("storage create requested: {}", unit.id);
        unit.config.validate()?;

        self.store.create(unit).await?;
        self.apply_create(unit).await?;
        info!("storage '{}' created successfully", unit.id);

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), NetworkError> {
        let unit = self.store.read(id).await?;

        self.apply_delete(&unit).await?;

        self.store.delete(id).await
    }

    pub async fn update(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        unit.config.validate()?;

        self.apply_update(unit).await?;

        self.store.update(unit).await
    }

    pub async fn list(&self) -> Result<Vec<NetworkUnit>, NetworkError> {
        self.store.list().await
    }
}

impl NetworkManager {
    async fn apply_create(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        debug!("applying create for network '{}'", unit.id);

        match &unit.config {
            NetworkConfig::Bridge { common: _, data } => {
                debug!("network '{}' is a bridge", unit.id);

                data.validate()?;
                let bridge = LinuxBridge::new(&data.bridge_name);

                bridge.create().await?;

                if let Some(parent) = &data.parent_iface {
                    bridge.attach_parent(parent).await?;
                }

                if data.stp {
                    bridge.enable_stp().await?;
                }

                if let Some(mtu) = data.mtu {
                    bridge.set_mtu(mtu).await?;
                }

                Ok(())
            }
            NetworkConfig::Nat { common, data } => {
                debug!("network '{}' is NAT", unit.id);

                data.validate()?;

                let nat = NatGateway::new(common, data);

                nat.ensure_bridge().await?;
                nat.enable_ip_forwarding().await?;
                nat.apply_iptables_rules().await?;

                Ok(())
            }
            NetworkConfig::MacVlan { common: _, data: _ } => {
                debug!("network '{}' is macvlan", unit.id);
                todo!()
                // data.validate()?;

                // let macvlan = MacVlanNetwork::new(&data.parent_iface, &data.mode);

                // macvlan.ensure_parent().await?;

                // Ok(())
            }
            NetworkConfig::None {} => {
                debug!("network '{}' is none", unit.id);
                todo!();
            }
        }
    }

    async fn apply_delete(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        debug!("applying delete for network '{}'", unit.id);
        match &unit.config {
            NetworkConfig::Bridge { common: _, data } => {
                let bridge = LinuxBridge::new(&data.bridge_name);

                if bridge.has_attached_ports().await? {
                    return Err(NetworkError::InUse(
                        "bridge still has attached interfaces".into(),
                    ));
                }

                bridge.delete().await
            }
            NetworkConfig::Nat { common, data } => {
                let nat = NatGateway::new(common, data);

                nat.remove_iptables_rules().await?;
                nat.delete_bridge().await
            }
            NetworkConfig::MacVlan { common: _, data: _ } => Ok(()),
            NetworkConfig::None {} => {
                debug!("network '{}' is none", unit.id);
                Ok(())
            }
        }
    }

    async fn apply_update(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        debug!("applying update for network '{}'", unit.id);
        match &unit.config {
            NetworkConfig::Bridge { common: _, data } => {
                let bridge = LinuxBridge::new(&data.bridge_name);

                if let Some(mtu) = data.mtu {
                    bridge.set_mtu(mtu).await?;
                }

                Ok(())
            }
            NetworkConfig::Nat { common: _, data: _ } => Err(NetworkError::UnsupportedOperation(
                "nat netwrok update is not supported".into(),
            )),
            NetworkConfig::MacVlan { common: _, data: _ } => {
                Err(NetworkError::UnsupportedOperation(
                    "macvlan netwrok update is not supported".into(),
                ))
            }
            NetworkConfig::None {} => {
                debug!("network '{}' is none", unit.id);
                Ok(())
            }
        }
    }
}
