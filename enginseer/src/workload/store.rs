use std::path::PathBuf;

use log::{debug, info, warn};
use tokio::fs;

use crate::workload::{configs::storage::WorkloadUnit, errors::VMError};

#[derive(Debug, Clone)]
pub struct WorkloadStore {
    base_dir: PathBuf,
}

impl WorkloadStore {
    pub fn new(root: PathBuf) -> Self {
        let base_dir = root.join("workloads");
        info!("initializing WorkloadStore at {:?}", base_dir);

        Self { base_dir }
    }

    fn path_for(&self, id: &str) -> PathBuf {
        self.base_dir.join(id).with_extension("toml")
    }
}

impl WorkloadStore {
    pub async fn read(&self, id: &str) -> Result<WorkloadUnit, VMError> {
        let path = self.path_for(id);
        debug!("reading workload  '{}'", id);

        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| VMError::IOError(path.clone(), err))?;

        let unit: WorkloadUnit = toml::from_str(&content)
            .map_err(|err| VMError::SerializationFailed(path, err.to_string()))?;

        Ok(unit)
    }

    pub async fn read_by_path(&self, path: &PathBuf) -> Result<WorkloadUnit, VMError> {
        debug!("reading workload by path '{:?}'", path);

        let content = fs::read_to_string(&path)
            .await
            .map_err(|err| VMError::IOError(path.clone(), err))?;

        let unit: WorkloadUnit = toml::from_str(&content)
            .map_err(|err| VMError::SerializationFailed(path.to_path_buf(), err.to_string()))?;

        Ok(unit)
    }

    pub async fn save(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        let path = self.path_for(&unit.id);
        let content = toml::to_string_pretty(&unit).map_err(|err| {
            VMError::InvalidConfig(format!("{}: {:?}", unit.id.clone(), err.to_string()))
        })?;

        fs::write(&path, content)
            .await
            .map_err(|err| VMError::SerializationFailed(path, err.to_string()))?;

        debug!("saved workload unit '{}'", unit.id);
        Ok(())
    }

    pub async fn create(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        info!("creating workload unit '{}'", unit.id);

        unit.config.validate()?;

        fs::create_dir_all(&self.base_dir)
            .await
            .map_err(|err| VMError::IOError(self.base_dir.clone(), err))?;

        let path = self.path_for(&unit.id);
        if path.exists() {
            warn!("workload unit '{}' already exist", unit.id);
            return Err(VMError::VMConfigAlreadyExist(path));
        }

        self.save(unit).await
    }

    pub async fn update(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        info!("update workload unit '{}'", unit.id);

        unit.config.validate()?;

        let path = self.path_for(&unit.id);
        if !path.exists() {
            warn!("workload unit '{}' already exist", unit.id);
            return Err(VMError::VMConfigDoesNotExist(path));
        }

        self.save(unit).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), VMError> {
        let path = &self.path_for(id);

        if !path.exists() {
            return Err(VMError::VMConfigDoesNotExist(path.to_path_buf()));
        }

        fs::remove_file(path)
            .await
            .map_err(|_| VMError::VMConfigRemove(path.to_path_buf()))?;

        info!("deleted workload unit '{}'", id);
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<WorkloadUnit>, VMError> {
        let mut result = Vec::new();

        let mut dir = fs::read_dir(&self.base_dir)
            .await
            .map_err(|err| VMError::IOError(self.base_dir.clone(), err))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|err| VMError::IOError(self.base_dir.clone(), err))?
        {
            let path = &entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .await
                .map_err(|err| VMError::IOError(path.to_path_buf(), err))?;

            let unit: WorkloadUnit =
                toml::from_str(&content).map_err(|err| VMError::InvalidConfig(err.to_string()))?;

            result.push(unit);
        }

        info!("listed {} workload units", result.len());
        Ok(result)
    }
}
