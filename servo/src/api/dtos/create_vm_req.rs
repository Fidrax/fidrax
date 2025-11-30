use std::path::PathBuf;

use chrono::Utc;
use enginseer::vm::configs::QemuConfig;
use regex::Regex;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{api::dtos::errors::DTOSErrors, errors::ServoErrors};

#[derive(Deserialize, ToSchema)]
pub struct CreateVMRequest {
    pub name: String,
    pub memory_mb: u64,
    pub vcpu: u8,
    pub disk_config_path: String,
}

impl CreateVMRequest {
    pub fn validate(&self) -> Result<(), DTOSErrors> {
        if self.name.trim().is_empty() {
            return Err(DTOSErrors::VMCreateRequestNameEmpty(
                "name cannot be empty".into(),
            ));
        }

        let name_re = Regex::new(r"^[A-Za-z0-9._-]+$").unwrap();
        if !name_re.is_match(&self.name) {
            return Err(DTOSErrors::VMCreateRequestInvalidName(self.name.clone()));
        }

        if self.memory_mb <= 0 {
            return Err(DTOSErrors::VMMemSizeInvalid(format!(
                "vm memory could not be zero or less {}",
                self.memory_mb
            )));
        }

        if self.vcpu <= 0 {
            return Err(DTOSErrors::VMCpuSizeInvalid("vm vcpu is invalid".into()));
        }

        if !self.disk_config_path.starts_with("/") {
            return Err(DTOSErrors::DiskPathStartPathInvalid(
                self.disk_config_path.clone(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<CreateVMRequest> for QemuConfig {
    type Error = ServoErrors;

    fn try_from(req: CreateVMRequest) -> Result<Self, Self::Error> {
        let _ = req.validate().map_err(|err| ServoErrors::DTOS(err));

        Ok(QemuConfig {
            name: req.name,
            memory_mb: req.memory_mb,
            vcpu: req.vcpu,
            disk_config_path: PathBuf::from(req.disk_config_path),
            created_at: Utc::now(),
        })
    }
}
