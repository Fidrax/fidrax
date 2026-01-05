use crate::{
    runtime::state::RuntimeState,
    storage::configs::storage::StorageUnit,
    workload::{configs::storage::WorkloadUnit, errors::VMError},
};

pub trait Runtime {
    async fn start(&self, unit: &WorkloadUnit, storages: &[StorageUnit]) -> Result<(), VMError>;
    async fn stop(&self, unit: &WorkloadUnit) -> Result<(), VMError>;
    async fn restart(&self, unit: &WorkloadUnit) -> Result<(), VMError>;
    async fn status(&self, unit: &WorkloadUnit) -> Result<RuntimeState, VMError>;
}
