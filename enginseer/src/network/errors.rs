use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("invalid network config {0:?}")]
    InvalidConfig(String),

    #[error("I/O error for {0:?}: {1}")]
    IOError(PathBuf, #[source] std::io::Error),

    #[error("serialization/deserialization failed for {0:?}: {1}")]
    SerializationFailed(PathBuf, String),

    #[error("network config does not exist {0:?}")]
    NetworkConfigDoesNotExist(PathBuf),

    #[error("network config remove error {0:?}")]
    NetworkConfigRemove(PathBuf),

    #[error("network config already exist {0:?}")]
    NetworkConfigAlreadyExist(PathBuf),

    #[error("network operation not supported {0:?}")]
    UnsupportedOperation(String),

    #[error("network resource is still in use {0:?}")]
    InUse(String),

    #[error("network command failed {0:?}")]
    CommandFailed(String),
}
