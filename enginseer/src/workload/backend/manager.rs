use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    runtime::{state::RuntimeState, traits::Runtime},
    storage::store::StorageStore,
    workload::{
        backend::{
            docker::runtime::DockerRuntime,
            qemu::{runtime::QemuRuntime, vm::QemuVM},
        },
        configs::storage::{WorkloadConfig, WorkloadUnit},
        errors::VMError,
    },
};

#[derive(Debug, Clone)]
pub enum WorkloadRuntime {
    Qemu(QemuRuntime),
    Docker(DockerRuntime),
}

#[derive(Clone)]
pub struct WorkloadManager {
    runtimes: HashMap<String, WorkloadRuntime>,
    storage_store: Arc<StorageStore>,
}

impl WorkloadManager {
    pub fn new(root: PathBuf, storage_store: StorageStore) -> Self {
        let mut runtimes = HashMap::new();

        runtimes.insert(
            "qemu".to_string(),
            WorkloadRuntime::Qemu(QemuRuntime {
                vm: QemuVM::new(root),
            }),
        );

        Self {
            runtimes,
            storage_store: Arc::new(storage_store),
        }
    }

    pub async fn start(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        unit.config.validate()?;

        let mut storages = Vec::new();
        for id in &unit.config.as_common().storages {
            let storage_unit = self.storage_store.read(id).await.map_err(|err| {
                VMError::InvalidConfig(format!(
                    "invalid config for storage of vm '{}' {}",
                    unit.id,
                    err.to_string()
                ))
            })?;
            storages.push(storage_unit);
        }

        match &unit.config {
            WorkloadConfig::VM { .. } => {
                if let Some(WorkloadRuntime::Qemu(runtime)) = self.runtimes.get("qemu") {
                    runtime.start(unit, &storages).await
                } else {
                    Err(VMError::RuntimenotFound("qemu".to_string()))
                }
            }
            WorkloadConfig::Docker { .. } => {
                if let Some(WorkloadRuntime::Docker(runtime)) = self.runtimes.get("docker") {
                    runtime.start(unit, &storages).await
                } else {
                    Err(VMError::RuntimenotFound("docker".to_string()))
                }
            }
            WorkloadConfig::LXC { .. } => {
                todo!()
            }
        }
    }

    pub async fn stop(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        match &unit.config {
            WorkloadConfig::VM { .. } => {
                if let Some(WorkloadRuntime::Qemu(runtime)) = self.runtimes.get("qemu") {
                    runtime.stop(unit).await
                } else {
                    Err(VMError::RuntimenotFound("qemu".to_string()))
                }
            }
            WorkloadConfig::Docker { .. } => {
                if let Some(WorkloadRuntime::Docker(runtime)) = self.runtimes.get("docker") {
                    runtime.stop(unit).await
                } else {
                    Err(VMError::RuntimenotFound("docker".to_string()))
                }
            }
            WorkloadConfig::LXC { .. } => {
                todo!()
            }
        }
    }

    pub async fn restart(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        match &unit.config {
            WorkloadConfig::VM { .. } => {
                if let Some(WorkloadRuntime::Qemu(runtime)) = self.runtimes.get("qemu") {
                    runtime.restart(unit).await
                } else {
                    Err(VMError::RuntimenotFound("qemu".to_string()))
                }
            }
            WorkloadConfig::Docker { .. } => {
                if let Some(WorkloadRuntime::Docker(runtime)) = self.runtimes.get("docker") {
                    runtime.restart(unit).await
                } else {
                    Err(VMError::RuntimenotFound("docker".to_string()))
                }
            }
            WorkloadConfig::LXC { .. } => {
                todo!()
            }
        }
    }

    pub async fn status(&self, unit: &WorkloadUnit) -> Result<RuntimeState, VMError> {
        match &unit.config {
            WorkloadConfig::VM { .. } => {
                if let Some(WorkloadRuntime::Qemu(runtime)) = self.runtimes.get("qemu") {
                    runtime.status(unit).await
                } else {
                    Err(VMError::RuntimenotFound("qemu".to_string()))
                }
            }
            WorkloadConfig::Docker { .. } => {
                if let Some(WorkloadRuntime::Docker(runtime)) = self.runtimes.get("docker") {
                    runtime.status(unit).await
                } else {
                    Err(VMError::RuntimenotFound("docker".to_string()))
                }
            }
            WorkloadConfig::LXC { .. } => {
                todo!()
            }
        }
    }
}
