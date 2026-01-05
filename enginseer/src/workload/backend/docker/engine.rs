use crate::{
    runtime::state::RuntimeState,
    storage::configs::storage::StorageUnit,
    workload::{configs::storage::WorkloadUnit, errors::VMError},
};

#[derive(Debug, Clone)]
pub struct DockerEngine {
    socket: String,
}

impl DockerEngine {
    pub fn new() -> Self {
        Self {
            socket: "/var/run/docker.sock".into(),
        }
    }

    pub async fn run_container(
        &self,
        unit: &WorkloadUnit,
        storages: &[StorageUnit],
    ) -> Result<(), VMError> {
        let (.., data) = unit.config.as_docker()?;

        let mut cmd = tokio::process::Command::new("docker");

        cmd.arg("run").arg("-d").arg("--name").arg(&unit.id);

        if let Some(envs) = &data.env {
            for e in envs {
                cmd.arg("-e").arg(e);
            }
        }

        for storage in storages {
            let (storage_common, storage_data) = storage.config.as_docker().map_err(|err| {
                VMError::InvalidConfig(format!(
                    "docker storage is not valid '{}', {}",
                    unit.id, err
                ))
            })?;

            let target = storage_data.target().map_err(|err| {
                VMError::InvalidConfig(format!(
                    "docker volume '{}' has no target: {}",
                    storage_common.name, err
                ))
            })?;

            let mut spec = format!("{}:{}", storage_common.name, target);

            if storage_data.readonly() {
                spec.push_str(":ro");
            }
        }

        cmd.arg(&data.image);

        if let Some(commands) = &data.command {
            for c in commands {
                cmd.arg("c");
            }
        }

        let status = cmd.status().await.map_err(|err| {
            VMError::CmdError("failed to start docker container".to_string(), err)
        })?;

        if !status.success() {
            return Err(VMError::FailedToStartContainer(format!(
                "failed to run docker container {}",
                unit.id
            )));
        }

        Ok(())
    }

    pub async fn stop_container(&self, id: &str) -> Result<(), VMError> {
        tokio::process::Command::new("docker")
            .arg("stop")
            .arg(id)
            .status()
            .await
            .map_err(|err| {
                VMError::FailedStopContainer(format!("unit '{}' {}", id, err.to_string()))
            })?;

        Ok(())
    }

    pub async fn restart_container(&self, id: &str) -> Result<(), VMError> {
        tokio::process::Command::new("docker")
            .arg("restart")
            .arg(id)
            .status()
            .await
            .map_err(|err| {
                VMError::FailedRestartContainer(format!("unit '{}' {}", id, err.to_string()))
            })?;

        Ok(())
    }

    pub async fn container_status(&self, id: &str) -> Result<RuntimeState, VMError> {
        let output = tokio::process::Command::new("docker")
            .arg("inspect")
            .arg("-f")
            .arg("{{.State.Status}}")
            .arg(id)
            .output()
            .await
            .map_err(|err| {
                VMError::FailedContainerStatus(format!("unit '{}' {}", id, err.to_string()))
            })?;

        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(match status.as_str() {
            "created" => RuntimeState::Defined,
            "running" => RuntimeState::Running,
            "exited" => RuntimeState::Exited,
            "dead" => RuntimeState::Failed {
                reason: "container dead".into(),
            },
            _ => RuntimeState::Unknown,
        })
    }
}
