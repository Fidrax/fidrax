use std::path::PathBuf;

use enginseer::disk::store::Qcow2DiskStore;
use enginseer::disk::{configs::DiskConfigEntry, manager::Qcow2DiskManager};

use crate::api::dtos::create_disk_req::CreateDiskRequest;
use crate::errors::ServoErrors;

#[derive(Debug, Clone)]
pub struct DiskService {
    manager: Qcow2DiskManager,
}

impl DiskService {
    pub fn new(root_path: PathBuf) -> Self {
        let store = Qcow2DiskStore::new(root_path.join("disks"));
        let manager = Qcow2DiskManager::new(store);

        Self { manager }
    }

    pub async fn create_disk(&self, req: CreateDiskRequest) -> Result<(), ServoErrors> {
        let entry: DiskConfigEntry = req.try_into()?;

        self.manager
            .create_disk(&entry)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn remove_disk(&self, name: &str) -> Result<(), ServoErrors> {
        self.manager
            .remove_disk(name)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn update_disk(&self, name: &str, new_size_gb: u64) -> Result<(), ServoErrors> {
        self.manager
            .update_disk(name, new_size_gb)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn list_disks(&self) -> Result<Vec<DiskConfigEntry>, ServoErrors> {
        self.manager
            .list_disks()
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }
}
