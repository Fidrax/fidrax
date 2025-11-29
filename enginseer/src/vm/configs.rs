use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::vm::errors::VMError;

#[derive(Debug, Clone)]
pub struct QemuConfig {
    pub name: String,
    pub memory_mb: u64,
    pub vcpu: u8,
    pub disk_config_path: PathBuf,
    pub created_at: DateTime<Utc>,
}

impl QemuConfig {
    pub fn validate(&self) -> Result<(), VMError> {
        if self.name.trim().is_empty() {
            return Err(VMError::InvalidConfig("vm name can not be empty".into()));
        }

        if self.memory_mb <= 0 {
            return Err(VMError::InvalidConfig(
                "vm memory can not be zero or negative".into(),
            ));
        }

        if self.vcpu <= 0 {
            return Err(VMError::InvalidConfig(
                "vm vcpu can not be zero or negative".into(),
            ));
        }

        if !self.disk_config_path.exists() {
            return Err(VMError::InvalidConfig("vm disk path does not exist".into()));
        }

        if self.created_at > Utc::now() {
            return Err(VMError::InvalidConfig(
                "vm created cannot be in future".into(),
            ));
        }

        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawQemuConfig {
    pub name: String,
    pub memory_mb: u64,
    pub vcpu: u8,
    pub disk_config_path: String,
    pub created_at: String,
}

impl TryFrom<RawQemuConfig> for QemuConfig {
    type Error = VMError;

    fn try_from(raw: RawQemuConfig) -> Result<Self, Self::Error> {
        let created_at = raw.created_at.parse::<DateTime<Utc>>().map_err(|_| {
            VMError::InvalidConfig(format!(
                "invalid created at config format {}",
                raw.created_at
            ))
        })?;

        Ok(QemuConfig {
            name: raw.name,
            memory_mb: raw.memory_mb,
            vcpu: raw.vcpu,
            disk_config_path: PathBuf::from(raw.disk_config_path),
            created_at,
        })
    }
}

impl From<QemuConfig> for RawQemuConfig {
    fn from(config: QemuConfig) -> Self {
        RawQemuConfig {
            name: config.name,
            memory_mb: config.memory_mb,
            vcpu: config.vcpu,
            disk_config_path: config.disk_config_path.to_string_lossy().to_string(),
            created_at: config.created_at.to_rfc3339(),
        }
    }
}
