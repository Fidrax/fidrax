use std::path::PathBuf;

use tokio::process::Command;

use crate::{
    disk::{configs::Qcow2DiskConfig, errors::DiskError, store::Qcow2DiskStore},
    traits::config::Config,
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
    disk_store: Qcow2DiskStore,
}

impl QemuVM {
    pub fn new(disk_store: Qcow2DiskStore) -> Self {
        Self { disk_store }
    }

    async fn disk_config(&self, config_path: &PathBuf) -> Result<Qcow2DiskConfig, DiskError> {
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
        let disk = match self.disk_config(&config.disk_config_path).await {
            Ok(disk) => disk,
            Err(err) => return Err(VMError::DiskError(err)),
        };

        let status = Command::new("qemu-system-x86_64")
            // name of vm
            .arg("-name")
            .arg(&config.name)
            // cpu
            .arg("-smp")
            .arg(config.vcpu.to_string())
            // memory
            .arg("-m")
            .arg(config.memory_mb.to_string())
            // disk
            .arg("-drive")
            .arg(format!(
                "file={},if=virtio,cache=none,aio=native",
                disk.disk_path.display()
            ))
            // qmp path
            .arg("-qmp")
            .arg(format!(
                "unix:{}/{},server=on,wait=off",
                "run/qmp", &config.name
            ))
            // run vm in background
            .arg("-daemonize")
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
