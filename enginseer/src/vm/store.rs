use crate::{
    traits::config::Config,
    vm::{
        configs::{QemuConfig, RawQemuConfig},
        errors::VMError,
    },
};
use chrono::Utc;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct QemuVMStore {
    base_dir: PathBuf,
}

impl QemuVMStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn get_config_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.base_dir.display(), name))
    }
}

impl Config<QemuConfig, VMError> for QemuVMStore {
    async fn read(&self, name: &str) -> Result<QemuConfig, VMError> {
        let config_path = self.base_dir.join(name).with_extension("yaml");

        let content = fs::read_to_string(&config_path)
            .await
            .map_err(|err| VMError::IOError(config_path.clone(), err))?;

        let raw_config: RawQemuConfig = serde_yaml::from_str(&content)
            .map_err(|err| VMError::SerializationFailed(config_path, err.to_string()))?;

        Ok(QemuConfig::try_from(raw_config).map_err(|err| {
            VMError::InvalidConfig(format!(
                "failed to convert raw config: {:?}",
                err.to_string()
            ))
        })?)
    }

    async fn read_by_path(&self, config_path: &PathBuf) -> Result<QemuConfig, VMError> {
        let content = fs::read_to_string(&config_path)
            .await
            .map_err(|err| VMError::IOError(config_path.clone(), err))?;

        let raw_config: RawQemuConfig = serde_yaml::from_str(&content).map_err(|err| {
            VMError::SerializationFailed(config_path.to_path_buf(), err.to_string())
        })?;

        Ok(QemuConfig::try_from(raw_config).map_err(|err| {
            VMError::InvalidConfig(format!(
                "failed to convert raw config: {:?}",
                err.to_string()
            ))
        })?)
    }

    async fn create(&self, config: &QemuConfig) -> Result<(), VMError> {
        config.validate()?;

        let config_path = self.base_dir.join(&config.name).with_extension("yaml");
        if config_path.exists() {
            return Err(VMError::VMConfigAlreadyExist(config_path));
        }

        let raw_config: RawQemuConfig = RawQemuConfig::from(config.clone());
        let content = serde_yaml::to_string(&raw_config).map_err(|err| {
            VMError::InvalidConfig(format!("{}: {:?}", config.name.clone(), err.to_string()))
        })?;

        fs::write(&config_path, content)
            .await
            .map_err(|err| VMError::SerializationFailed(config_path, err.to_string()))?;

        Ok(())
    }

    async fn update(&self, config: &QemuConfig) -> Result<(), VMError> {
        config.validate()?;

        let config_path = self.base_dir.join(&config.name).with_extension("yaml");
        if !config_path.exists() {
            return Err(VMError::NotFound(
                config_path.clone().to_string_lossy().to_string(),
            ));
        }

        let raw_config: RawQemuConfig = RawQemuConfig::from(config.clone());
        let content = serde_yaml::to_string(&raw_config).map_err(|err| {
            VMError::InvalidConfig(format!("{}: {:?}", config.name.clone(), err.to_string()))
        })?;

        fs::write(&config_path, content)
            .await
            .map_err(|err| VMError::SerializationFailed(config_path, err.to_string()))?;

        Ok(())
    }

    async fn delete(&self, name: &str) -> Result<(), VMError> {
        let file = self.base_dir.join(name).with_extension("yaml");

        if !file.exists() {
            return Err(VMError::NotFound(
                file.clone().to_string_lossy().to_string(),
            ));
        }

        let _ = fs::remove_file(&file)
            .await
            .map_err(|err| VMError::IOError(file, err));

        Ok(())
    }

    async fn list(&self) -> Result<Vec<QemuConfig>, VMError> {
        let mut configs: Vec<QemuConfig> = Vec::new();

        let mut dir = fs::read_dir(&self.base_dir)
            .await
            .map_err(|err| VMError::IOError(self.base_dir.clone(), err))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|err| VMError::IOError(self.base_dir.clone(), err))?
        {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .await
                .map_err(|err| VMError::IOError(path, err))?;

            let raw: RawQemuConfig = serde_yaml::from_str(&content)
                .map_err(|err| VMError::InvalidConfig(err.to_string()))?;

            let config: QemuConfig = raw.try_into().map_err(|err| err)?;

            configs.push(config);
        }

        Ok(configs)
    }
}
