use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::disk::errors::DiskError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Qcow2DiskAllocationMode {
    Sparse,
    Full,
    Metadata,
}

impl TryFrom<String> for Qcow2DiskAllocationMode {
    type Error = DiskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sparse" => Ok(Self::Sparse),
            "full" => Ok(Self::Full),
            "metadata" => Ok(Self::Metadata),
            other=> Err(DiskError::InvalidAllocationMode(format!("invalid allocation mode '{other}'"))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawQcow2DiskConfig {
    pub name: String,
    pub path: String,
    pub size_gb: u64,
    pub allocation_mode: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Qcow2DiskConfig {
    pub name: String,
    pub path: PathBuf,
    pub size_gb: u64,
    pub allocation_mode: Qcow2DiskAllocationMode,
    pub created_at: DateTime<Utc>,
}

impl Qcow2DiskConfig {
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

        if self.created_at > chrono::Utc::now() {
            return Err(DiskError::InvalidConfig(
                "created at cannot be in the future".into(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<RawQcow2DiskConfig> for Qcow2DiskConfig {
    type Error = DiskError;

    fn try_from(raw: RawQcow2DiskConfig) -> Result<Self, Self::Error> {
        let allocation_mode = match raw.allocation_mode.to_lowercase().as_str() {
            "sparse" => Qcow2DiskAllocationMode::Sparse,
            "full" => Qcow2DiskAllocationMode::Full,
            "metadata" => Qcow2DiskAllocationMode::Metadata,
            other => return Err(DiskError::InvalidAllocationMode(other.to_string())),
        };

        let created_at = raw
            .created_at
            .parse::<DateTime<Utc>>()
            .map_err(|_| DiskError::InvalidConfigDate(raw.created_at.to_string()))?;

        Ok(Qcow2DiskConfig {
            name: raw.name,
            path: PathBuf::from(raw.path),
            size_gb: raw.size_gb,
            allocation_mode: allocation_mode,
            created_at,
        })
    }
}

impl From<Qcow2DiskConfig> for RawQcow2DiskConfig {
    fn from(config: Qcow2DiskConfig) -> Self {
        let allocation_mode = match config.allocation_mode {
            Qcow2DiskAllocationMode::Sparse => "Sparse",
            Qcow2DiskAllocationMode::Full => "Full",
            Qcow2DiskAllocationMode::Metadata => "Metadata",
        }
        .to_string();

        RawQcow2DiskConfig {
            name: config.name,
            path: config.path.to_string_lossy().to_string(),
            size_gb: config.size_gb,
            allocation_mode,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct DiskConfigEntry {
    pub path: PathBuf,
    pub config: Qcow2DiskConfig,
}

