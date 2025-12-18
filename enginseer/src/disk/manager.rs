use crate::{
    disk::{configs::Qcow2DiskConfig, errors::DiskError, qcow2::Qcow2Disk, store::Qcow2DiskStore},
    traits::config::Config,
};

#[derive(Debug, Clone)]
pub struct Qcow2DiskManager {
    pub store: Qcow2DiskStore,
}

impl Qcow2DiskManager {
    pub fn new(store: Qcow2DiskStore) -> Self {
        Self { store }
    }

    pub async fn create_disk(&self, config: &Qcow2DiskConfig) -> Result<(), DiskError> {
        self.store.create(config).await?;

        let disk = Qcow2Disk::new(&config.path);
        disk.create_disk(&config.allocation_mode, &config.size_gb)
            .await?;

        Ok(())
    }

    pub async fn remove_disk(&self, name: &str) -> Result<(), DiskError> {
        let config = self.store.read(name).await?;

        let disk = Qcow2Disk::new(config.path);
        disk.remove_disk().await?;

        self.store.delete(name).await?;

        Ok(())
    }

    pub async fn update_disk(&self, name: &str, new_size_gb: u64) -> Result<(), DiskError> {
        let mut config = self.store.read(name).await?;

        config.size_gb = new_size_gb;
        let disk = Qcow2Disk::new(config.path.clone());
        disk.resize_disk(new_size_gb).await?;
        self.store.update(&config).await?;

        Ok(())
    }

    pub async fn list_disks(&self) -> Result<Vec<Qcow2DiskConfig>, DiskError> {
        self.store.list().await
    }
}
