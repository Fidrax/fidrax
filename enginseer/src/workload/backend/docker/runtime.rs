use log::debug;

use crate::{
    network::configs::storage::NetworkUnit, runtime::traits::Runtime, storage::configs::storage::StorageUnit, workload::{
        backend::docker::engine::DockerEngine, configs::storage::WorkloadUnit, errors::VMError,
    }
};

#[derive(Debug, Clone)]
pub struct DockerRuntime {
    engine: DockerEngine,
}

impl DockerRuntime {
    pub fn new(engine: DockerEngine) -> Self {
        Self { engine }
    }
}

impl Runtime for DockerRuntime {
    async fn start(&self, unit: &WorkloadUnit, storages: &[StorageUnit], networks: &[NetworkUnit]) -> Result<(), VMError> {
        debug!("docker run time start '{}'", unit.id);
        
        self.engine.run_container(unit, storages, networks).await
    }

    async fn stop(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        debug!("docker run time stop '{}'", unit.id);

        self.engine.stop_container(&unit.id).await
    }

    async fn restart(&self, unit: &WorkloadUnit) -> Result<(), VMError> {
        debug!("docker run time restart '{}'", unit.id);

        self.engine.restart_container(&unit.id).await
    }

    async fn status(
        &self,
        unit: &WorkloadUnit,
    ) -> Result<crate::runtime::state::RuntimeState, VMError> {
        debug!("docker run time status '{}'", unit.id);

        self.engine.container_status(&unit.id).await
    }
}
