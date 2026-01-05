use log::{debug, info, warn};
use std::path::PathBuf;
use tokio::fs::{self};

use crate::network::{configs::storage::NetworkUnit, errors::NetworkError};

#[derive(Debug, Clone)]
pub struct NetworkStore {
    base_dir: PathBuf,
}

impl NetworkStore {
    pub fn new(root: PathBuf) -> Self {
        let base_dir = root.join("network");
        info!("initializing NetworkStore at {:?}", base_dir);
        Self { base_dir }
    }

    pub fn path_for(&self, id: &str) -> PathBuf {
        self.base_dir.join(id).with_extension("toml")
    }

    pub async fn read(&self, id: &str) -> Result<NetworkUnit, NetworkError> {
        let path = self.path_for(id);
        debug!("reading network unit '{}'", id);

        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| NetworkError::IOError(path.clone(), err))?;

        let unit: NetworkUnit = toml::from_str(&content)
            .map_err(|err| NetworkError::SerializationFailed(path.clone(), err.to_string()))?;

        Ok(unit)
    }

    pub async fn read_by_path(&self, path: &PathBuf) -> Result<NetworkUnit, NetworkError> {
        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| NetworkError::IOError(path.clone(), err))?;

        let unit: NetworkUnit = toml::from_str(&content).map_err(|err| {
            NetworkError::SerializationFailed(path.to_path_buf(), err.to_string())
        })?;

        Ok(unit)
    }

    pub async fn save(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        let path = self.path_for(&unit.id);
        let content = toml::to_string_pretty(&unit).map_err(|err| {
            NetworkError::InvalidConfig(format!("{}: {:?}", unit.id.clone(), err.to_string()))
        })?;

        fs::write(&path, content)
            .await
            .map_err(|err| NetworkError::SerializationFailed(path, err.to_string()))?;

        debug!("saved storage unit '{}'", unit.id);
        Ok(())
    }

    pub async fn create(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        info!("creating storage unit '{}'", unit.id);

        unit.config.validate()?;

        fs::create_dir_all(&self.base_dir)
            .await
            .map_err(|err| NetworkError::IOError(self.base_dir.clone(), err))?;

        let path = self.path_for(&unit.id);
        if path.exists() {
            warn!("storage unit '{}' already exist", unit.id);
            return Err(NetworkError::NetworkConfigAlreadyExist(path));
        }

        self.save(unit).await
    }

    pub async fn update(&self, unit: &NetworkUnit) -> Result<(), NetworkError> {
        info!("update storage unit '{}'", unit.id);

        unit.config.validate()?;

        let path = self.path_for(&unit.id);
        if !path.exists() {
            warn!("storage unit '{}' already exist", unit.id);
            return Err(NetworkError::NetworkConfigDoesNotExist(path));
        }

        self.save(unit).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), NetworkError> {
        let path = &self.path_for(id);

        if !path.exists() {
            return Err(NetworkError::NetworkConfigDoesNotExist(path.to_path_buf()));
        }

        fs::remove_file(path)
            .await
            .map_err(|_| NetworkError::NetworkConfigRemove(path.to_path_buf()))?;

        info!("deleted storage unit '{}'", id);
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<NetworkUnit>, NetworkError> {
        let mut result = Vec::new();

        let mut dir = fs::read_dir(&self.base_dir)
            .await
            .map_err(|err| NetworkError::IOError(self.base_dir.clone(), err))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|err| NetworkError::IOError(self.base_dir.clone(), err))?
        {
            let path = &entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .await
                .map_err(|err| NetworkError::IOError(path.to_path_buf(), err))?;

            let unit: NetworkUnit = toml::from_str(&content)
                .map_err(|err| NetworkError::InvalidConfig(err.to_string()))?;

            result.push(unit);
        }

        info!("listed {} storage units", result.len());
        Ok(result)
    }
}
