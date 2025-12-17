use enginseer::vm::configs::{QemuConfig};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize,  ToSchema, Debug, Clone)]
pub struct ResponseQemuConfig {
    pub name: String,
    pub memory_mb: u64,
    pub vcpu: u8,
    pub disk_config_path: String,
    pub created_at: String,
}

impl From<QemuConfig> for ResponseQemuConfig {
    fn from(cfg: QemuConfig) -> Self {
        ResponseQemuConfig {
            name: cfg.name,
            memory_mb: cfg.memory_mb,
            vcpu: cfg.vcpu,
            disk_config_path: cfg.disk_config_path.to_string_lossy().to_string(),
            created_at: cfg.created_at.to_rfc3339(),
        }
    }
}
