
use log::debug;

use crate::runtime::state::RuntimeState;
use crate::storage::configs::storage::StorageUnit;
use crate::workload::configs::storage::WorkloadUnit;
use crate::workload::{backend::qemu::vm::QemuVM, errors::VMError};
use crate::runtime::traits::Runtime;

#[derive(Debug, Clone)]
pub struct QemuRuntime {
    pub vm: QemuVM,
}

impl Runtime for QemuRuntime {
    async fn start(&self, unit: &WorkloadUnit, storages: &[StorageUnit]) -> Result<(), VMError> {
        unit.config.validate()?;

        debug!("qemu runtime start");
        self.vm.start(unit, storages).await
    }

    async fn stop(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        unit.config.validate()?;

        debug!("qemu runtime stop");
        self.vm.shutdown(unit).await
    }

    async fn restart(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        unit.config.validate()?;

        debug!("qemu runtime restart");
        self.vm.restart(unit).await
    }

    async fn status(&self, unit: &WorkloadUnit) -> Result<RuntimeState, VMError> {
        unit.config.validate()?;

        debug!("qemu runtime status");
        self.vm.status(unit).await
    }
}
