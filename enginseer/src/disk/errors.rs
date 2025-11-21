use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiskError {
    #[error("failed to create disk at {0:?}: {1}")]
    CreationFailed(PathBuf, #[source] std::io::Error),

    #[error("failed to create disk at {0}")]
    DiskCreationFailed(String),

    #[error("disk new size is invalid {0}")]
    DiskNewSize(String),

    #[error("disk resize failed {0:?}: {1}")]
    ResizeFailed(PathBuf, #[source] std::io::Error),

    #[error("disk not found at path {0:?}")]
    NotFound(PathBuf),

    #[error("qemu-img failed for disk {path:?}: {message}")]
    QemuImageCommandFailed { path: PathBuf, message: String },

    #[error("failed to destroy disk at {0:?}: {1}")]
    DestroyFailed(PathBuf, #[source]std::io::Error),

    #[error("invalid path {0:?}")]
    InvalidPath(PathBuf),

    #[error("I/O error for {0:?}: {1}")]
    IOError(PathBuf, #[source] std::io::Error),

    #[error("serialization/deserialization failed for {0:?}: {1}")]
    SerializationFailed(PathBuf, String),

    #[error("unsupported disk format {0:?}")]
    UnsupportedDiskFormat(String),

    #[error("disk config date {0:?}")]
    InvalidConfigDate(String),

    #[error("disk allocation {0:?}")]
    InvalidAllocationMode(String),

    #[error("invalid disk config {0:?}")]
    InvalidConfig(String),

    #[error("disk already exist {0:?}")]
    DiskAlreadyExist(PathBuf),

    #[error("disk config already exist {0:?}")]
    DiskConfigAlreadyExist(PathBuf),

    #[error("disk config does not exist {0:?}")]
    DiskConfigDoesNotExist(PathBuf),

    #[error("disk config remove error {0:?}")]
    DiskConfigRemove(PathBuf),
}
