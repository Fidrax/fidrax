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
    pub disk_path: String,
    pub size_gb: u64,
    pub allocation_mode: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Qcow2DiskConfig {
    pub name: String,
    pub disk_path: PathBuf,
    pub size_gb: u64,
    pub allocation_mode: Qcow2DiskAllocationMode,
    pub created_at: DateTime<Utc>,
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
            disk_path: PathBuf::from(raw.disk_path),
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
            disk_path: config.disk_path.to_string_lossy().to_string(),
            size_gb: config.size_gb,
            allocation_mode,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}
