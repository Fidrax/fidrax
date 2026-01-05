use std::path::PathBuf;

use crate::network::errors::NetworkError;

pub struct LinuxBridge {
    pub name: String,
}

impl LinuxBridge {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub async fn create(&self) -> Result<(), NetworkError> {
        let status = tokio::process::Command::new("ip")
            .arg("link")
            .arg("add")
            .arg(&self.name)
            .arg("type")
            .arg("bridge")
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        if !status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "failed to create bridge {}",
                self.name
            )))?;
        }

        // bring bridge up
        let _ = tokio::process::Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(&self.name)
            .arg("up")
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        Ok(())
    }

    pub async fn delete(&self) -> Result<(), NetworkError> {
        let status = tokio::process::Command::new("ip")
            .arg("link")
            .arg("delete")
            .arg(&self.name)
            .arg("type")
            .arg("bridge")
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        if !status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "failed to delete bridge {}",
                self.name
            )));
        }

        Ok(())
    }

    pub async fn attach_parent(&self, iface: &str) -> Result<(), NetworkError> {
        let status = tokio::process::Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(iface)
            .arg("master")
            .arg(&self.name)
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        if !status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "failed to attach interface {} to bridge {}",
                iface, self.name
            )));
        }

        Ok(())
    }

    pub async fn enable_stp(&self) -> Result<(), NetworkError> {
        let status = tokio::process::Command::new("bridge")
            .arg("stp")
            .arg(&self.name)
            .arg("on")
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        if !status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "failed to enable STP on bridge {}",
                self.name
            )));
        }

        Ok(())
    }

    pub async fn set_mtu(&self, mtu: u32) -> Result<(), NetworkError> {
        let status = tokio::process::Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(&self.name)
            .arg("mtu")
            .arg(mtu.to_string())
            .status()
            .await
            .map_err(|err| NetworkError::CommandFailed(err.to_string()))?;

        if !status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "failed to set MTU {} on bridge {}",
                mtu, self.name
            )));
        }

        Ok(())
    }

    pub async fn has_attached_ports(&self) -> Result<bool, NetworkError> {
        let path = format!("/sys/class/net/{}/brif", self.name);
        let mut entries = tokio::fs::read_dir(&path)
            .await
            .map_err(|err| NetworkError::IOError(PathBuf::from(&path), err))?;

        let mut has_ports = false;
        while let Some(_) = entries
            .next_entry()
            .await
            .map_err(|err| NetworkError::IOError(PathBuf::from(&path), err))?
        {
            has_ports = true;
            break;
        }

        Ok(has_ports)
    }
}
