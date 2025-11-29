use crate::{traits::config::Config, vm::{backend::qemu::{state::VMState, vm::QemuVM}, configs::QemuConfig, errors::VMError, store::QemuVMStore}};

pub struct QemuManager{
    store: QemuVMStore,
    vm: QemuVM,
}

impl QemuManager{
    async fn create(&self, config: &QemuConfig) -> Result<(), VMError> {
        self.store.create(config).await
    }

    async fn start(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.start(&config).await
    }

    async fn shutdown(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.shutdown(&config).await
    }

    async fn restart(&self, name: &String) -> Result<(), VMError> {
        let config = self.store.read(name).await?;

        self.vm.restart(&config).await
    }

    async fn status(&self, name: &String) -> Result<VMState, VMError> {
        let config = self.store.read(name).await?;

        self.vm.status(&config).await
    }
}

