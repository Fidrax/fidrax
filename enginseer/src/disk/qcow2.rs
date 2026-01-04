use std::path::PathBuf;
use tokio::{fs, process::Command};

use crate::disk::{configs::storage::Qcow2AllocationMode, errors::DiskError};

#[derive(Debug)]
pub struct Qcow2Disk {
    pub path: PathBuf,
}

impl Qcow2Disk {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    pub async fn create_disk(
        &self,
        alloc_mode: &Qcow2AllocationMode,
        size_gb: &u64,
    ) -> Result<(), DiskError> {
        if self.path().exists() {
            return Err(DiskError::DiskAlreadyExist(self.path().to_path_buf()));
        }

        let mut cmd = Command::new("qemu-img");

        cmd.arg("create").arg("-f").arg("qcow2");

        match alloc_mode {
            Qcow2AllocationMode::Sparse => {
                // default is sparse
            }
            Qcow2AllocationMode::Full => {
                cmd.arg("-o").arg("preallocation=full");
            }
            Qcow2AllocationMode::Metadata => {
                cmd.arg("-o").arg("preallocation=metadata");
            }
        }

        // convert GB to byte
        let disk_size = size_gb * 1024 * 1024 * 1024;
        cmd.arg(self.path.to_str().unwrap())
            .arg(disk_size.to_string());

        let status = cmd
            .status()
            .await
            .map_err(|err| DiskError::IOError(self.path().to_path_buf(), err))?;

        if !status.success() {
            return Err(DiskError::DiskCreationFailed(format!(
                "qemu-img exited with code {:?} for disk '{}'",
                status.code(),
                self.path().display()
            )));
        }

        Ok(())
    }

    pub async fn remove_disk(&self) -> Result<(), DiskError> {
        if !self.path().exists() {
            return Ok(());
        }

        fs::remove_file(&self.path())
            .await
            .map_err(|err| DiskError::DestroyFailed(self.path().to_path_buf(), err))?;
        Ok(())
    }

    pub async fn resize_disk(&self, new_size_gb: u64) -> Result<(), DiskError> {
        if new_size_gb <= 0 {
            return Err(DiskError::DiskNewSize(format!(
                "new size is {:?}",
                new_size_gb
            )));
        }

        if !self.path().exists() {
            return Err(DiskError::NotFound(self.path().to_path_buf()));
        }

        let new_size_bytes = new_size_gb * 1024 * 1024 * 1024;

        let mut cmd = Command::new("qemu-img");
        cmd.arg("resize")
            .arg(self.path().to_str().unwrap())
            .arg(new_size_bytes.to_string());

        let status = cmd
            .status()
            .await
            .map_err(|err| DiskError::ResizeFailed(self.path().to_path_buf(), err))?;

        if !status.success() {
            return Err(DiskError::DiskCreationFailed(format!(
                "qemu-img resize failed (exit code {:?}) for disk '{}'",
                status.code(),
                self.path().display()
            )));
        }

        Ok(())
    }
}
