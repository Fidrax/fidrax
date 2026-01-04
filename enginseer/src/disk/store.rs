use log::{debug, info, warn};
use std::path::PathBuf;
use tokio::fs::{self};

use crate::disk::{configs::storage::StorageUnit, errors::DiskError};

#[derive(Debug, Clone)]
pub struct StorageStore {
    base_dir: PathBuf,
}

impl StorageStore {
    pub fn new(base_dir: PathBuf) -> Self {
        info!("initializing StorageStore at {:?}", base_dir);
        Self { base_dir }
    }

    pub fn path_for(&self, id: &str) -> PathBuf {
        self.base_dir.join(id).with_extension("toml")
    }

    pub async fn read(&self, id: &str) -> Result<StorageUnit, DiskError> {
        let path = self.path_for(id);
        debug!("reading storage unit '{}'", id);

        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| DiskError::IOError(path.clone(), err))?;

        let unit: StorageUnit = toml::from_str(&content)
            .map_err(|err| DiskError::SerializationFailed(path.clone(), err.to_string()))?;

        Ok(unit)
    }

    pub async fn read_by_path(&self, path: &PathBuf) -> Result<StorageUnit, DiskError> {
        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| DiskError::IOError(path.clone(), err))?;

        let unit: StorageUnit = toml::from_str(&content)
            .map_err(|err| DiskError::SerializationFailed(path.to_path_buf(), err.to_string()))?;

        Ok(unit)
    }

    pub async fn save(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        let path = self.path_for(&unit.id);
        let content = toml::to_string_pretty(&unit).map_err(|err| {
            DiskError::InvalidConfig(format!("{}: {:?}", unit.id.clone(), err.to_string()))
        })?;

        fs::write(&path, content)
            .await
            .map_err(|err| DiskError::SerializationFailed(path, err.to_string()))?;

        debug!("saved storage unit '{}'", unit.id);
        Ok(())
    }

    pub async fn create(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        info!("creating storage unit '{}'", unit.id);

        unit.config.validate()?;

        fs::create_dir_all(&self.base_dir)
            .await
            .map_err(|err| DiskError::IOError(self.base_dir.clone(), err))?;

        let path = self.path_for(&unit.id);
        if path.exists() {
            warn!("storage unit '{}' already exist", unit.id);
            return Err(DiskError::DiskConfigAlreadyExist(path));
        }

        self.save(unit).await
    }

    pub async fn update(&self, unit: &StorageUnit) -> Result<(), DiskError> {
        info!("update storage unit '{}'", unit.id);

        unit.config.validate()?;

        let path = self.path_for(&unit.id);
        if !path.exists() {
            warn!("storage unit '{}' already exist", unit.id);
            return Err(DiskError::DiskConfigDoesNotExist(path));
        }

        self.save(unit).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), DiskError> {
        let path = &self.path_for(id);

        if !path.exists() {
            return Err(DiskError::DiskConfigDoesNotExist(path.to_path_buf()));
        }

        fs::remove_file(path)
            .await
            .map_err(|_| DiskError::DiskConfigRemove(path.to_path_buf()));

        info!("deleted storage unit '{}'", id);
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<StorageUnit>, DiskError> {
        let mut result = Vec::new();

        let mut dir = fs::read_dir(&self.base_dir)
            .await
            .map_err(|err| DiskError::IOError(self.base_dir.clone(), err))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|err| DiskError::IOError(self.base_dir.clone(), err))?
        {
            let path = &entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .await
                .map_err(|err| DiskError::IOError(path.to_path_buf(), err))?;

            let unit: StorageUnit = toml::from_str(&content)
                .map_err(|err| DiskError::InvalidConfig(err.to_string()))?;

            result.push(unit);
        }

        info!("listed {} storage units", result.len());
        Ok(result)
    }
}
