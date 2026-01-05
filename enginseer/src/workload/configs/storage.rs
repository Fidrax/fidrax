use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::workload::errors::VMError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkloadMetadata {
    pub created_at: DateTime<Utc>,
    pub last_modified: Option<DateTime<Utc>>,
    pub version: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkloadCommon {
    pub id: String,
    pub name: String,
    pub metadata: WorkloadMetadata,

    pub memory_mb: u64,
    pub vcpu: u8,

    // reference to storage, network
    pub storages: Vec<String>,  // StorageUnit IDs
    pub networks: Vec<String>, // Network IDs
}

impl WorkloadCommon {
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

        // TODO revise checking existent of storage
        // for disk in &self.storage {
        //     if !disk.exists() {
        //         return Err(VMError::InvalidConfig(format!(
        //             "vm disk path does not exist for disk {:?}",
        //             disk
        //         )));
        //     }
        // }

        if self.metadata.created_at > Utc::now() {
            return Err(VMError::InvalidConfig(
                "vm created cannot be in future".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuData {
    // TODO need to make them as enum
    pub machine: Option<String>,   // q35, pc
    pub cpu_model: Option<String>, // host, skylake
    pub accel: Option<String>,     // kvm, tcg
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerData {
    pub image: String,
    pub command: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LxcData {
    pub template: String,
    pub distro: String,
    pub release: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WorkloadConfig {
    VM {
        #[serde(flatten)]
        common: WorkloadCommon,
        #[serde(flatten)]
        data: QemuData,
    },
    Docker {
        #[serde(flatten)]
        common: WorkloadCommon,
        #[serde(flatten)]
        data: DockerData,
    },
    LXC {
        #[serde(flatten)]
        common: WorkloadCommon,
        #[serde(flatten)]
        data: LxcData,
    },
    // future: wasm, firecracker ...
}

impl WorkloadConfig {
    pub fn validate(&self) -> Result<(), VMError> {
        match self {
            Self::VM { common, .. } => common.validate(),
            Self::Docker { common, .. } => common.validate(),
            Self::LXC { common, .. } => common.validate(),
        }
    }

    pub fn as_common(&self) -> &WorkloadCommon {
        match self {
            WorkloadConfig::VM { common, .. }
            | WorkloadConfig::Docker { common, .. }
            | WorkloadConfig::LXC { common, .. } => common,
        }
    }

    pub fn as_vm(&self) -> Result<(&WorkloadCommon, &QemuData), VMError> {
        match self {
            WorkloadConfig::VM { common, data } => Ok((common, data)),
            _ => Err(VMError::InvalidConfig("config is not for a vm".to_string())),
        }
    }

    pub fn as_docker(&self) -> Result<(&WorkloadCommon, &DockerData), VMError> {
        match self {
            WorkloadConfig::Docker { common, data } => Ok((common, data)),
            _ => Err(VMError::InvalidConfig("config is not for a vm".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkloadUnit {
    pub id: String,
    pub config: WorkloadConfig,
}

// tests
#[cfg(test)]
mod tests {
    // require to write tests
}
