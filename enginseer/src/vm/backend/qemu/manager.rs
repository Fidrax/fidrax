use std::path::PathBuf;

use crate::{disk::store::Qcow2DiskStore, traits::config::Config, vm::{backend::qemu::{state::VMState, vm::QemuVM}, configs::QemuConfig, errors::VMError, store::QemuVMStore}};

#[derive(Debug, Clone)]
pub struct QemuManager{
    store: QemuVMStore,
    vm: QemuVM,
}

impl QemuManager{
    pub fn new(vm_base_dir: PathBuf, disk_base_dir: PathBuf) -> Self {
        let store = QemuVMStore::new(vm_base_dir);
        let disk_store = Qcow2DiskStore::new(disk_base_dir);

        let vm = QemuVM::new(disk_store);
        Self { store: store, vm: vm }
    }

    pub async fn create(&self, config: &QemuConfig) -> Result<(), VMError> {
        self.store.create(config).await
    }

    pub async fn start(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.start(&config).await
    }

    pub async fn shutdown(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.shutdown(&config).await
    }

    pub async fn restart(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.restart(&config).await
    }

    pub async fn status(&self, name: &String) -> Result<VMState, VMError> {
        let config = self.store.read(name).await?;

        self.vm.status(&config).await
    }

    pub async fn list(&self) -> Result<Vec<QemuConfig>, VMError> {
        self.store.list().await
    }
}

