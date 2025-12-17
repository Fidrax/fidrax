use enginseer::disk::configs::{Qcow2DiskAllocationMode, Qcow2DiskConfig};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize,  ToSchema, Debug, Clone)]
pub struct ResponseQcow2DiskConfig {
    pub name: String,
    pub disk_path: String,
    pub size_gb: u64,
    pub allocation_mode: String,
    pub created_at: String,
}

impl From<Qcow2DiskConfig> for ResponseQcow2DiskConfig {
    fn from(cfg: Qcow2DiskConfig) -> Self {
        ResponseQcow2DiskConfig {
            name: cfg.name,
            disk_path: cfg.disk_path.to_string_lossy().to_string(),
            size_gb: cfg.size_gb,
            allocation_mode: match cfg.allocation_mode {
                Qcow2DiskAllocationMode::Sparse => "Sparse".to_string(),
                Qcow2DiskAllocationMode::Full => "Full".to_string(),
                Qcow2DiskAllocationMode::Metadata => "Metadata".to_string(),
            },
            created_at: cfg.created_at.to_rfc3339(),
        }
    }
}
