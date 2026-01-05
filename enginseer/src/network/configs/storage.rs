use ::serde::{Deserialize, Serialize};

use crate::network::errors::NetworkError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkCommon {
    pub name: String,
    pub bridge_name: String,  // linux bridge name e.g br-nat0
    pub cidr: Option<String>, // "10.0.0.0/24"
    pub gateway: Option<String>,
    pub mac: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BridgeData {
    // name of the linux bridge device(e.g "br0", "br-workload")
    pub bridge_name: String,

    // optional parent interface("eth0")
    pub parent_iface: Option<String>,

    // enable STP on the bridge
    #[serde(default)]
    pub stp: bool,

    // option MTU override
    pub mtu: Option<u32>,
}

impl BridgeData {
    pub fn validate(&self) -> Result<(), NetworkError> {
        if self.bridge_name.trim().is_empty() {
            return Err(NetworkError::InvalidConfig(
                "bridge_name cannot be empty".into(),
            ));
        }

        if let Some(mtu) = self.mtu {
            if mtu < 576 {
                return Err(NetworkError::InvalidConfig("mtu too small".into()));
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

fn default_tcp() -> Protocol {
    Protocol::Tcp
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortForward {
    pub host_port: u16,
    pub guest_ip: String,
    pub guest_port: u16,
    #[serde(default = "default_tcp")]
    pub protocol: Protocol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NatData {
    // outgoing interface on the host (e.g "eth0")
    pub outbound_iface: String,

    // enable SNAT/MASQUERADE
    #[serde(default = "default_true")]
    pub masquerade: bool,

    // enable IPv6 NAT (optional, future use)
    #[serde(default)]
    pub enable_ipv6: bool,

    // optional port forwarding (host -> workload)
    #[serde(default)]
    pub port_forwards: Vec<PortForward>,
}

impl NatData {
    pub fn validate(&self) -> Result<(), NetworkError> {
        if self.outbound_iface.trim().is_empty() {
            return Err(NetworkError::InvalidConfig(
                "outbound_iface cannot be empty".into(),
            ));
        }
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacVlanData {
    // Host interface to attach to (e.g "eth0")
    pub parent_iface: String,

    // Macvlan operating mode
    pub mode: MacVlanMode,

    // optional static MAC address
    pub mac_address: Option<String>,
}

impl MacVlanData {
    pub fn validate(&self) -> Result<(), NetworkError> {
        if self.parent_iface.trim().is_empty() {
            return Err(NetworkError::InvalidConfig(
                "parent_iface cannot be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MacVlanMode {
    Bridge,
    Private,
    Vepa,
    Passthru,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum NetworkConfig {
    Bridge {
        common: NetworkCommon,
        data: BridgeData,
    },
    Nat {
        common: NetworkCommon,
        data: NatData,
    },
    MacVlan {
        common: NetworkCommon,
        data: MacVlanData,
    },
    // future SRIOV...
    None{},
}

impl NetworkConfig {
    pub fn validate(&self) -> Result<(), NetworkError> {
        match self {
            Self::Nat { common, data } => {
                // common and data validate?
                todo!()
            }
            NetworkConfig::Bridge { common, data } => todo!(),
            NetworkConfig::Nat { common, data } => todo!(),
            NetworkConfig::MacVlan { common, data } => todo!(),
            NetworkConfig::None {  } => todo!()
        }
    }
}

#[derive(Debug)]
pub struct VmNetAttachment {
    pub tap_name: String,
    pub bridge: String,
    pub mac: Option<String>,
}

impl VmNetAttachment {
    pub fn none() -> Self {
        Self {
            tap_name: String::new(),
            bridge: String::new(),
            mac: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkUnit {
    pub id: String,
    pub config: NetworkConfig,
}

impl NetworkUnit {
    pub fn as_vm(&self) -> Result<VmNetAttachment, NetworkError> {
        match &self.config {
            NetworkConfig::Bridge { common, .. } | NetworkConfig::Nat { common, .. } => {
                Ok(VmNetAttachment {
                    tap_name: format!("tap-{}", self.id),
                    bridge: common.bridge_name.clone(),
                    mac: common.mac.clone(),
                })
            }
            NetworkConfig::MacVlan { .. } => Err(NetworkError::UnsupportedOperation(
                "macvlan not supported for qemu".into(),
            )),
            NetworkConfig::None { .. } => Ok(VmNetAttachment::none()),
        }
    }
}
