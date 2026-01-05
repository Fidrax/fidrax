use enginseer::workload::configs::{QemuConfig};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize,  ToSchema, Debug, Clone)]
pub struct ResponseQemuConfig {
    pub name: String,
    pub memory_mb: u64,
    pub vcpu: u8,
    pub disks: Vec<String>,
    pub created_at: String,
}

impl From<QemuConfig> for ResponseQemuConfig {
    fn from(cfg: QemuConfig) -> Self {
        let mut disks: Vec<String> = Vec::new();
        for disk in &cfg.disks {
            disks.push(disk.to_string_lossy().to_string());
        }

        ResponseQemuConfig {
            name: cfg.name,
            memory_mb: cfg.memory_mb,
            vcpu: cfg.vcpu,
            disks: disks, 
            created_at: cfg.created_at.to_rfc3339(),
        }
    }
}
