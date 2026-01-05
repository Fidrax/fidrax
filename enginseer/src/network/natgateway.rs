use log::debug;

use crate::network::{
    configs::storage::{NatData, NetworkCommon},
    errors::NetworkError,
};

#[derive(Debug, Clone)]
pub struct NatGateway {
    bridge_name: String,
    subnet: Option<String>,
    data: NatData,
}

impl NatGateway {
    pub fn new(common: &NetworkCommon, data: &NatData) -> Self {
        Self {
            bridge_name: common.bridge_name.clone(),
            subnet: common.cidr.clone(),
            data: data.clone(),
        }
    }

    pub async fn ensure_bridge(&self) -> Result<(), NetworkError> {
        debug!("ensure bridge '{}'", self.bridge_name);

        tokio::process::Command::new("ip")
            .args(["link", "add", "name", &self.bridge_name, "type", "bridge"])
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        tokio::process::Command::new("ip")
            .args(["link", "set", &self.bridge_name, "up"])
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        Ok(())
    }

    pub async fn enable_ip_forwarding(&self) -> Result<(), NetworkError> {
        debug!("enable ip forwarding '{}'", self.bridge_name);

        tokio::process::Command::new("sysctl")
            .args(["-w", "net.ip4.ip_forward=1"])
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        Ok(())
    }

    pub async fn apply_iptables_rules(&self) -> Result<(), NetworkError> {
        debug!("apply ip tables rules for '{}'", self.bridge_name);

        if self.data.masquerade {
            tokio::process::Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-A",
                    "PREROUTING",
                    "-o",
                    &self.data.outbound_iface,
                    "-j",
                    "MASQUERADE",
                ])
                .status()
                .await
                .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;
        }

        for pf in &self.data.port_forwards {
            tokio::process::Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-A",
                    "PREROUTING",
                    "-p",
                    "tcp",
                    "--dport",
                    &pf.host_port.to_string(),
                    "-j",
                    "DNAT",
                    "--to-destination",
                    &format!("{}:{}", pf.guest_ip, pf.guest_port),
                ])
                .status()
                .await
                .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;
        }

        Ok(())
    }

    pub async fn remove_iptables_rules(&self) -> Result<(), NetworkError> {
        if !self.data.masquerade {
            return Ok(());
        }

        let bridge = &self.bridge_name;
        let iface = &self.data.outbound_iface;

        let br = &format!("{}+", bridge);
        let rules = vec![
            vec![
                "iptables",
                "-t",
                "nat",
                "-D",
                "POSTROUTING",
                "-s",
                br,
                "-o",
                iface,
                "-j",
                "MASQUERADE",
            ],
            vec![
                "iptables", "-D", "FORWARD", "-i", bridge, "-o", iface, "-j", "ACCEPT",
            ],
            vec![
                "iptables",
                "-D",
                "FORWARD",
                "-i",
                iface,
                "-o",
                bridge,
                "-m",
                "state",
                "--state",
                "RELATED,ESTABLISHED",
                "-j",
                "ACCEPT",
            ],
        ];

        for rule in rules {
            let status = tokio::process::Command::new(rule[0])
                .args(&rule[1..])
                .status()
                .await;

            if let Err(err) = status {
                debug!("iptables rule removal skipped: {}", err);
            }
        }

        Ok(())
    }

    pub async fn delete_bridge(&self) -> Result<(), NetworkError> {
        let br = &self.bridge_name;

        let down = tokio::process::Command::new("ip")
            .args(["link", "set", br, "down"])
            .status()
            .await;

        if let Err(err) = down {
            debug!("bridge '{}' already down: {}", br, err);
        }

        let del = tokio::process::Command::new("ip")
            .args(["link", "del", br])
            .status()
            .await;

        match del {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(NetworkError::CommandFailed(format!(
                "failed to delete bridge '{}'",
                br
            ))),
            Err(err) => Err(NetworkError::CommandFailed(format!("ip link del {}", err))),
        }
    }
}
