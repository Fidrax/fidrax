use log::{debug, info};

use crate::storage::{
    configs::storage::{StorageConfig, StorageUnit, VMDiskFormat},
    docker_volume::DockerVolume,
    errors::DiskError,
    qcow2::Qcow2Disk,
    store::StorageStore,
};

#[derive(Debug, Clone)]
pub struct StorageManager {
    pub store: StorageStore,
}

impl StorageManager {
    pub fn new(store: StorageStore) -> Self {
        Self { store }
    }

    pub async fn create(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        info!("storage create requested: {}", unit.id);
        unit.config.validate()?;

        self.store.create(unit).await?;
        self.apply_create(unit).await?;
        info!("storage '{}' created successfully", unit.id);

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), DiskError> {
        let unit = self.store.read(id).await?;

        self.apply_delete(&unit).await?;

        self.store.delete(id).await
    }

    pub async fn update(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        unit.config.validate()?;

        self.apply_update(unit).await?;

        self.store.update(unit).await
    }

    pub async fn list(&self) -> Result<Vec<StorageUnit>, DiskError> {
        self.store.list().await
    }
}

impl StorageManager {
    async fn apply_create(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        debug!("applying create for storage '{}'", unit.id);

        match &unit.config {
            StorageConfig::VMDisk { common, data } => {
                debug!("storage '{}' is a VM disk", unit.id);
                match &data.format {
                    VMDiskFormat::Qcow2 { allocation_mode } => {
                        let qcow2 = Qcow2Disk::new(&common.path);
                        qcow2.create_disk(allocation_mode, &common.size_gb).await
                    }
                    _ => Ok(()), // future formats
                }
            }
            StorageConfig::DockerVolume { common, data } => {
                // require implementation
                debug!("storage '{}' is a docker volume", unit.id);
                let volume = DockerVolume::new(&common.name);
                volume.create(&data.driver, &data.mount_option).await
            }
        }
    }

    async fn apply_delete(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        debug!("applying delete for storage '{}'", unit.id);
        match &unit.config {
            StorageConfig::VMDisk { common, data } => {
                debug!("storage '{}' is a VM disk", unit.id);
                match &data.format {
                    VMDiskFormat::Qcow2 { allocation_mode: _ } => {
                        let qcow2 = Qcow2Disk::new(&common.path);
                        qcow2.remove_disk().await
                    }
                    _ => Ok(()), // future formats
                }
            }
            StorageConfig::DockerVolume { common, .. } => {
                debug!("storage '{}' is a docker volume", unit.id);
                // require implementation
                let volume = DockerVolume::new(&common.name);
                volume.delete().await
            }
        }
    }

    async fn apply_update(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        debug!("applying update for storage '{}'", unit.id);
        match &unit.config {
            StorageConfig::VMDisk { common, data } => {
                debug!("storage '{}' is a VM disk", unit.id);
                if let VMDiskFormat::Qcow2 { allocation_mode: _ } = data.format {
                    let qcow2 = Qcow2Disk::new(&common.path);
                    qcow2.resize_disk(common.size_gb).await
                } else {
                    Ok(())
                }
            }
            StorageConfig::DockerVolume { .. } => {
                debug!("storage '{}' is a docker volume", unit.id);
                // require implementation
                info!("docker volume update is not implemented yet!!");
                Err(DiskError::UnsupportedOperation(
                    "docker volume update is not supported".to_string(),
                ))
            }
        }
    }
}
