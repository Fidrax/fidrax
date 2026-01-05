use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::storage::errors::DiskError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageMetadata {
    pub created_at: DateTime<Utc>,
    pub last_modified: Option<DateTime<Utc>>,
    pub version: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageCommon {
    pub name: String,
    pub path: PathBuf,
    pub size_gb: u64,
    pub metadata: StorageMetadata,
}

impl StorageCommon {
    pub fn validate(&self) -> Result<(), DiskError> {
        if self.name.trim().is_empty() {
            return Err(DiskError::InvalidConfig("name can not be empty".into()));
        }

        let name_re = regex::Regex::new(r"^[A-Za-z0-9._-]+$").unwrap();
        if !name_re.is_match(&self.name) {
            return Err(DiskError::InvalidConfig(format!(
                "invalid name '{}', allowed characters: letters, numbers, '.', '_', '-'",
                self.name
            )));
        }

        if self.size_gb <= 0 {
            return Err(DiskError::InvalidConfig(
                "disk size must be greater than 0 GB".into(),
            ));
        }

        if self.metadata.created_at > chrono::Utc::now() {
            return Err(DiskError::InvalidConfig(
                "created at cannot be in the future".into(),
            ));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Qcow2AllocationMode {
    Sparse,
    Full,
    Metadata,
}

impl TryFrom<String> for Qcow2AllocationMode {
    type Error = DiskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sparse" => Ok(Self::Sparse),
            "full" => Ok(Self::Full),
            "metadata" => Ok(Self::Metadata),
            other => Err(DiskError::InvalidAllocationMode(format!(
                "invalid allocation mode '{other}'"
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VMDiskData {
    pub format: VMDiskFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VMDiskFormat {
    Qcow2 {
        allocation_mode: Qcow2AllocationMode,
    },
    Raw,
    VMDK,
    VHD,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerVolumeData {
    pub driver: String, // should be enum for "local", "overlay", ...
    pub mount_option: Option<Vec<String>>, // e.g "size=10G"
}

impl DockerVolumeData {
    pub fn target(&self) -> Result<&str, DiskError> {
        if let Some(opts) = &self.mount_option {
            for opt in opts {
                if let Some(t) = opt.strip_prefix("target=") {
                    return Ok(t);
                }
            }
        }
        Err(DiskError::InvalidConfig(
            "docker volume missing target=".to_string(),
        ))
    }

    pub fn readonly(&self) -> bool {
        self.mount_option
            .as_ref()
            .map(|opts| opts.iter().any(|o| o == "ro"))
            .unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum StorageConfig {
    VMDisk {
        #[serde(flatten)]
        common: StorageCommon,
        #[serde(flatten)]
        data: VMDiskData,
    },
    DockerVolume {
        #[serde(flatten)]
        common: StorageCommon,
        #[serde(flatten)]
        data: DockerVolumeData,
    },
    // LXC etc...
}

impl StorageConfig {
    pub fn validate(&self) -> Result<(), DiskError> {
        match self {
            Self::VMDisk { common, .. } => common.validate(),
            Self::DockerVolume { common, .. } => common.validate(),
        }
    }

    pub fn as_vm(&self) -> Result<(&StorageCommon, &VMDiskData), DiskError> {
        match self {
            StorageConfig::VMDisk { common, data } => Ok((common, data)),
            _ => Err(DiskError::InvalidConfig(
                "storage config is not for vm".to_string(),
            )),
        }
    }

    pub fn as_docker(&self) -> Result<(&StorageCommon, &DockerVolumeData), DiskError> {
        match self {
            StorageConfig::DockerVolume { common, data } => Ok((common, data)),
            _ => Err(DiskError::InvalidConfig(
                "storage config is not for docker".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageUnit {
    pub id: String, // stable id for snapshot/reference
    pub config: StorageConfig,
}

// tests
#[cfg(test)]
mod tests {
    // require to write tests
}
