use std::path::PathBuf;

use enginseer::vm::{
    backend::qemu::{manager::QemuManager, state::VMState},
    configs::QemuConfig,
};

use crate::{api::dtos::create_vm_req::CreateVMRequest, errors::ServoErrors};

#[derive(Debug, Clone)]
pub struct QemuVMService {
    manager: QemuManager,
}

impl QemuVMService {
    pub fn new(vm_base_dir: PathBuf, disk_base_dir: PathBuf) -> Self {
        let manager = QemuManager::new(vm_base_dir, disk_base_dir);
        Self { manager: manager }
    }

    pub async fn create_vm(&self, req: CreateVMRequest) -> Result<(), ServoErrors> {
        let config: QemuConfig = req.try_into()?;

        self.manager
            .create(&config)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn start(&self, name: &String) -> Result<(), ServoErrors> {
        self.manager
            .start(name)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn shutdown(&self, name: &String) -> Result<(), ServoErrors> {
        self.manager
            .shutdown(name)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn restart(&self, name: &String) -> Result<(), ServoErrors> {
        self.manager
            .restart(name)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn status(&self, name: &String) -> Result<VMState, ServoErrors> {
        self.manager
            .status(name)
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }

    pub async fn list(&self) -> Result<Vec<QemuConfig>, ServoErrors> {
        self.manager
            .list()
            .await
            .map_err(|err| ServoErrors::EnginseerErrors(err.into()))
    }
}
