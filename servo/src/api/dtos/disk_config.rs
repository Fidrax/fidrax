use enginseer::disk::configs::{Qcow2DiskAllocationMode, DiskConfigEntry};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize,  ToSchema, Debug, Clone)]
pub struct ResponseQcow2DiskConfig {
    pub name: String,
    pub path: String,
    pub size_gb: u64,
    pub allocation_mode: String,
    pub created_at: String,
}


#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct ResponseDiskConfigEntry {
    path: String,
    config: ResponseQcow2DiskConfig,
}

impl From<DiskConfigEntry> for ResponseDiskConfigEntry {
    fn from(entry: DiskConfigEntry) -> Self {
        Self {
            path: entry.path.to_string_lossy().to_string(),
            config: ResponseQcow2DiskConfig {
                name: entry.config.name,
                path: entry.config.path.to_string_lossy().to_string(),
                size_gb: entry.config.size_gb,
                allocation_mode: match entry.config.allocation_mode {
                    Qcow2DiskAllocationMode::Sparse => "Sparse".to_string(),
                    Qcow2DiskAllocationMode::Full => "Full".to_string(),
                    Qcow2DiskAllocationMode::Metadata => "Metadata".to_string(),
                },
                created_at: entry.config.created_at.to_rfc3339(),
            }
        }
    }
}
