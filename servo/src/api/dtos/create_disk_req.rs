use chrono::Utc;
use enginseer::disk::configs::{Qcow2DiskAllocationMode, Qcow2DiskConfig};
use regex::Regex;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::errors::ServoErrors;

use crate::api::dtos::errors::DTOSErrors;

#[derive(Deserialize, ToSchema)]
pub struct CreateDiskRequest {
    pub name: String,
    pub size_gb: u64,
    pub disk_path: String,
    pub alloc_mode: String,
}

impl CreateDiskRequest {
    pub fn validate(&self) -> Result<(), DTOSErrors> {
        if self.name.trim().is_empty() {
            return Err(DTOSErrors::DiskCreateRequestNameEmpty(
                "name cannot be empty".into(),
            ));
        }

        let name_re = Regex::new(r"^[A-Za-z0-9._-]+$").unwrap();
        if !name_re.is_match(&self.name) {
            return Err(DTOSErrors::DiskCreateRequestInvalidName(self.name.clone()));
        }

        if self.size_gb <= 0 {
            return Err(DTOSErrors::DiskSizeInvalid(format!(
                "disk could not be zero or less {}",
                self.size_gb
            )));
        }

        if self.disk_path.trim().is_empty() {
            return Err(DTOSErrors::DiskPathEmpty("disk path is empty".into()));
        }

        if !self.disk_path.starts_with("/") {
            return Err(DTOSErrors::DiskPathStartPathInvalid(self.disk_path.clone()));
        }

        match self.alloc_mode.clone().to_lowercase().as_str() {
            "sparse" | "full" | "metadata" => (),
            other => {
                return Err(DTOSErrors::DiskInvalidAllocMode(other.into()));
            }
        };

        Ok(())
    }
}

impl TryFrom<CreateDiskRequest> for Qcow2DiskConfig {
    type Error = ServoErrors;

    fn try_from(req: CreateDiskRequest) -> Result<Self, Self::Error> {
        let _ = req.validate().map_err(|err| ServoErrors::DTOS(err));
        let alloc_mode = Qcow2DiskAllocationMode::try_from(req.alloc_mode)
            .map_err(|e| ServoErrors::DTOS(DTOSErrors::Disk(e)))?;

        Ok(Qcow2DiskConfig {
            name: req.name,
            disk_path: req.disk_path.into(),
            size_gb: req.size_gb,
            allocation_mode: alloc_mode,
            created_at: Utc::now(),
        })
    }
}
