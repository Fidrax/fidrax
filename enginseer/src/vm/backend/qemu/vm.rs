use std::path::PathBuf;

use tokio::process::Command;

use crate::{
    disk::{
        configs::storage::{StorageConfig, StorageUnit},
        errors::DiskError,
        store::StorageStore,
    },
    vm::{
        backend::qemu::{
            qmp::{
                client::QmpClient,
                cmd::{
                    query_status::{QemuStatus, QueryStatusRequest},
                    system_powerdown::SystemPowerdown,
                    system_reset::SystemReset,
                },
            },
            state::VMState,
        },
        configs::QemuConfig,
        errors::VMError,
    },
};

#[derive(Debug, Clone)]
pub struct QemuVM {
    disk_store: StorageStore,
}

impl QemuVM {
    pub fn new(disk_store: StorageStore) -> Self {
        Self { disk_store }
    }

    async fn disk_config(&self, config_path: &PathBuf) -> Result<StorageUnit, DiskError> {
        Ok(self.disk_store.read_by_path(config_path).await?)
    }

    async fn qmp_connect(&self, config: &QemuConfig) -> Result<QmpClient, VMError> {
        let socket = PathBuf::from("run/qmp").join(&config.name);
        Ok(QmpClient::connect(socket)
            .await
            .map_err(|err| VMError::QmpClient(config.name.clone(), err))?)
    }
}

// impl VirtualMachine<&QemuConfig> for QemuVM {
impl QemuVM {
    pub async fn start(&self, config: &QemuConfig) -> Result<(), VMError> {
        let mut disks = Vec::new();

        for disk in &config.disks {
            let unit = self
                .disk_config(disk)
                .await
                .map_err(|err| VMError::DiskError(err))?;

            let path = match &unit.config {
                StorageConfig::VMDisk { common, data } => &common.path,
                _ => {
                    return Err(VMError::DiskError(DiskError::InvalidConfig(
                        "vm disk config is invalid".to_string(),
                    )));
                }
            };

            let arg = format!("file={},if=virtio,cache=none,aio=native", path.display());

            disks.push(arg);
        }

        let mut cmd = Command::new("qemu-system-x86_64");
        cmd
            // name of vm
            .arg("-name")
            .arg(&config.name)
            // cpu
            .arg("-smp")
            .arg(config.vcpu.to_string())
            // memory
            .arg("-m")
            .arg(config.memory_mb.to_string());

        for drive in disks {
            cmd
                // disk
                .arg("-drive")
                .arg(drive);
        }

        cmd
            // qmp path
            .arg("-qmp")
            .arg(format!(
                "unix:{}/{},server=on,wait=off",
                "run/qmp", &config.name
            ))
            // run vm in background
            .arg("-daemonize");

        let status = cmd
            .status()
            .await
            .map_err(|err| VMError::CmdError(config.name.clone(), err))?;

        if !status.success() {
            return Err(VMError::QemuStartFailed(config.name.clone(), status.code()));
        }

        Ok(())
    }

    pub async fn shutdown(&self, config: &QemuConfig) -> Result<(), VMError> {
        let mut qmp = self.qmp_connect(config).await?;

        qmp.execute(SystemPowerdown)
            .await
            .map_err(|err| VMError::QmpClient(config.name.clone(), err))?;

        Ok(())
    }

    pub async fn restart(&self, config: &QemuConfig) -> Result<(), VMError> {
        let mut qmp = self.qmp_connect(config).await?;

        qmp.execute(SystemReset)
            .await
            .map_err(|err| VMError::QmpClient(config.name.clone(), err))?;

        Ok(())
    }

    pub async fn status(&self, config: &QemuConfig) -> Result<VMState, VMError> {
        let mut qmp = self.qmp_connect(config).await?;

        match qmp.execute(QueryStatusRequest).await {
            Ok(resp) => match resp.status {
                QemuStatus::Running => Ok(VMState::Running),
                QemuStatus::Paused => Ok(VMState::Paused),
                QemuStatus::Shutdown | QemuStatus::Cold => Ok(VMState::Stopped),
                _ => Ok(VMState::Unknown),
            },
            Err(err) => return Err(VMError::QmpClient(config.name.clone(), err)),
        }
    }

    async fn pause(&self) -> Result<(), VMError> {
        todo!()
    }
}
