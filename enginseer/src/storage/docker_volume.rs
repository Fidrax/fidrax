use tokio::process::Command;

use log::{debug, info, warn};

use crate::storage::errors::DiskError;

#[derive(Debug)]
pub struct DockerVolume {
    name: String,
}

impl DockerVolume {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    fn name(&self) -> &str {
        &self.name
    }

    pub async fn create(
        &self,
        driver: &str,
        options: &Option<Vec<String>>,
    ) -> Result<(), DiskError> {
        info!("creating docker volume '{}'", self.name());

        let mut cmd = Command::new("docker");
        cmd.arg("volume").arg("create").arg("--driver").arg(driver);

        if let Some(options) = options {
            for opt in options {
                cmd.arg("--opt").arg(opt);
            }
        }

        cmd.arg(self.name());

        let status = cmd
            .status()
            .await
            .map_err(|err| DiskError::DockerVolumeFailed(err.to_string()))?;

        if !status.success() {
            return Err(DiskError::DockerVolumeStatus(
                "failed to create docker volume".to_string(),
                status.code(),
            ));
        }

        debug!("docker volume '{}' created", self.name());
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), DiskError> {
        info!("deleting docker volume '{}'", self.name());

        let status = Command::new("docker")
            .arg("volume")
            .arg("rm")
            .arg(self.name())
            .status()
            .await
            .map_err(|err| {
                DiskError::DockerVolumeDeleteFailed(self.name().to_owned(), err.to_string())
            })?;

        if !status.success() {
            warn!(
                "docker volume '{}' removal return non-zero status {:?}",
                self.name(),
                status.code()
            );
        }
        Ok(())
    }
}
