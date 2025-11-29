use std::path::PathBuf;
use tokio::fs::{self};

use crate::{
    disk::{
        configs::{Qcow2DiskConfig, RawQcow2DiskConfig},
        errors::DiskError,
    },
    traits::config::Config,
};

#[derive(Debug, Clone)]
pub struct Qcow2DiskStore {
    base_dir: PathBuf,
}

impl Qcow2DiskStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn get_config_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.base_dir.display(), name))
    }
}

impl Config<Qcow2DiskConfig, DiskError> for Qcow2DiskStore {
    async fn read(&self, name: &str) -> Result<Qcow2DiskConfig, DiskError> {
        let config_path = self.base_dir.join(name).with_extension("yaml");

        let content = fs::read_to_string(&config_path)
            .await
            .map_err(|err| DiskError::IOError(config_path.clone(), err))?;

        let raw_config: RawQcow2DiskConfig = serde_yaml::from_str(&content)
            .map_err(|err| DiskError::SerializationFailed(config_path, err.to_string()))?;

        Ok(Qcow2DiskConfig::try_from(raw_config).map_err(|e| {
            DiskError::InvalidConfig(format!("failed to convert raw config: {:?}", e.to_string()))
        })?)
    }

    async fn read_by_path(&self, config_path: &PathBuf) -> Result<Qcow2DiskConfig, DiskError> {
        let content = fs::read_to_string(&config_path)
            .await
            .map_err(|err| DiskError::IOError(config_path.clone(), err))?;

        let raw_config: RawQcow2DiskConfig = serde_yaml::from_str(&content)
            .map_err(|err| DiskError::SerializationFailed(config_path.to_path_buf(), err.to_string()))?;

        Ok(Qcow2DiskConfig::try_from(raw_config).map_err(|e| {
            DiskError::InvalidConfig(format!("failed to convert raw config: {:?}", e.to_string()))
        })?)
    }

    async fn create(&self, config: &Qcow2DiskConfig) -> Result<(), DiskError> {
        config.validate()?;

        let config_path = self.base_dir.join(&config.name).with_extension("yaml");
        if config_path.exists() {
            return Err(DiskError::DiskConfigAlreadyExist(config_path));
        }

        let raw_config: RawQcow2DiskConfig = RawQcow2DiskConfig::from(config.clone());
        let content = serde_yaml::to_string(&raw_config).map_err(|err| {
            DiskError::InvalidConfig(format!("{}: {:?}", config.name.clone(), err.to_string()))
        })?;

        fs::write(&config_path, content)
            .await
            .map_err(|err| DiskError::SerializationFailed(config_path, err.to_string()))?;

        Ok(())
    }

    async fn update(&self, config: &Qcow2DiskConfig) -> Result<(), DiskError> {
        config.validate()?;

        let config_path = self.base_dir.join(&config.name).with_extension("yaml");
        if !config_path.exists() {
            return Err(DiskError::DiskConfigDoesNotExist(config_path));
        }

        let raw_config: RawQcow2DiskConfig = RawQcow2DiskConfig::from(config.clone());
        let content = serde_yaml::to_string(&raw_config).map_err(|err| {
            DiskError::InvalidConfig(format!("{}: {:?}", config.name.clone(), err.to_string()))
        })?;

        fs::write(&config_path, content)
            .await
            .map_err(|err| DiskError::SerializationFailed(config_path, err.to_string()))?;

        Ok(())
    }

    async fn delete(&self, name: &str) -> Result<(), DiskError> {
        let file = &self.get_config_path(name);

        if !file.exists() {
            return Err(DiskError::DiskConfigDoesNotExist(file.to_path_buf()));
        }

        let _ = fs::remove_file(file)
            .await
            .map_err(|_| DiskError::DiskConfigRemove(file.to_path_buf()));

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Qcow2DiskConfig>, DiskError> {
        let mut configs: Vec<Qcow2DiskConfig> = Vec::new();

        let mut dir = fs::read_dir(&self.base_dir)
            .await
            .map_err(|err| DiskError::IOError(self.base_dir.clone(), err))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|err| DiskError::IOError(self.base_dir.clone(), err))?
        {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .await
                .map_err(|err| DiskError::IOError(path, err))?;

            let raw: RawQcow2DiskConfig = serde_yaml::from_str(&content)
                .map_err(|err| DiskError::InvalidConfig(err.to_string()))?;

            let config: Qcow2DiskConfig = raw.try_into().map_err(|err| err)?;

            configs.push(config);
        }

        Ok(configs)
    }

}
