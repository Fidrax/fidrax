use std::path::PathBuf;

use log::info;
use tokio::process::Command;

use crate::{
    network::configs::storage::NetworkUnit,
    runtime::state::RuntimeState,
    storage::configs::storage::StorageUnit,
    workload::{
        backend::qemu::qmp::{
            client::QmpClient,
            cmd::{
                query_status::{QemuStatus, QueryStatusRequest},
                system_powerdown::SystemPowerdown,
                system_reset::SystemReset,
            },
        },
        configs::storage::WorkloadUnit,
        errors::VMError,
    },
};

#[derive(Debug, Clone)]
pub struct QemuVM {
    base_runtime: PathBuf,
}

impl QemuVM {
    pub fn new(root: PathBuf) -> Self {
        // TODO make sure that path is created
        let base_runtime = root.join("qemu").join("qmp");
        info!("qemu vm initializing at path '{:?}'", base_runtime);
        Self { base_runtime }
    }

    async fn qmp_connect(&self, unit: &WorkloadUnit) -> Result<QmpClient, VMError> {
        let socket = self.base_runtime.join(&unit.id);
        Ok(QmpClient::connect(socket)
            .await
            .map_err(|err| VMError::QmpClient(unit.id.clone(), err))?)
    }
}

// impl VirtualMachine<&QemuConfig> for QemuVM {
impl QemuVM {
    pub async fn start(
        &self,
        unit: &WorkloadUnit,
        storages: &[StorageUnit],
        networks: &[NetworkUnit],
    ) -> Result<(), VMError> {
        let (vm_common, ..) = unit.config.as_vm()?;
        let mut disk_args = Vec::new();

        let mut cmd = Command::new("qemu-system-x86_64");
        cmd
            // name of vm
            .arg("-name")
            .arg(&vm_common.name)
            // cpu
            .arg("-smp")
            .arg(vm_common.vcpu.to_string())
            // memory
            .arg("-m")
            .arg(vm_common.memory_mb.to_string());

        for storage in storages {
            let (storage_common, ..) = storage.config.as_vm().map_err(|err| {
                VMError::InvalidConfig(format!("vm storage is not valid '{}', {}", unit.id, err))
            })?;

            let arg = format!(
                "file={},if=virtio,cache=none,aio=native",
                storage_common.path.display()
            );

            disk_args.push(arg);
        }

        for disk in disk_args {
            cmd
                // disk
                .arg("-drive")
                .arg(disk);
        }

        // network
        for (idx, net) in networks.iter().enumerate() {
            let net = net.as_vm().map_err(|err| {
                VMError::InvalidConfig(format!("invalid vm network '{}': {}", unit.id, err))
            })?;

            let net_id = format!("net{}", idx);

            cmd.arg("-netdev").arg(format!(
                "tap,id={},ifname={},script=no,downscript=no",
                net_id, net.tap_name
            ));

            let mut dev = format!("virito-net-pci,netdev={}", net_id);

            if let Some(mac) = &net.mac {
                dev.push_str(&format!(",mac={}", mac));
            }

            cmd.arg("-device").arg(dev);
        }

        cmd
            // qmp path
            .arg("-qmp")
            .arg(format!(
                "unix:{}/{},server=on,wait=off",
                self.base_runtime.display(),
                &unit.id
            ))
            // run vm in background
            .arg("-daemonize");

        let status = cmd
            .status()
            .await
            .map_err(|err| VMError::CmdError(unit.id.clone(), err))?;

        if !status.success() {
            return Err(VMError::QemuStartFailed(unit.id.clone(), status.code()));
        }

        Ok(())
    }

    pub async fn shutdown(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        let mut qmp = self.qmp_connect(unit).await?;

        qmp.execute(SystemPowerdown)
            .await
            .map_err(|err| VMError::QmpClient(unit.id.clone(), err))?;

        Ok(())
    }

    pub async fn restart(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        let mut qmp = self.qmp_connect(unit).await?;

        qmp.execute(SystemReset)
            .await
            .map_err(|err| VMError::QmpClient(unit.id.clone(), err))?;

        Ok(())
    }

    pub async fn status(&self, unit: &WorkloadUnit) -> Result<RuntimeState, VMError> {
        let mut qmp = self.qmp_connect(unit).await?;

        match qmp.execute(QueryStatusRequest).await {
            Ok(resp) => match resp.status {
                QemuStatus::Running => Ok(RuntimeState::Running),
                QemuStatus::Paused => Ok(RuntimeState::Paused),
                QemuStatus::Shutdown | QemuStatus::Cold => Ok(RuntimeState::Shutdown),
                _ => Ok(RuntimeState::Unknown),
            },
            Err(err) => return Err(VMError::QmpClient(unit.id.clone(), err)),
        }
    }

    // async fn pause(&self) -> Result<(), VMError> {
    //     todo!()
    // }
}
