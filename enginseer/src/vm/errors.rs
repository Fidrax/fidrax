use std::path::PathBuf;

use thiserror::Error;

use crate::disk::errors::DiskError;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("invalid vm config {0:?}")]
    InvalidConfig(String),

    #[error("vm config not found {0:?}")]
    NotFound(String),

    #[error("I/O error for {0:?}: {1}")]
    IOError(PathBuf, #[source] std::io::Error),

    #[error("serialization/deserialization failed for {0:?}: {1}")]
    SerializationFailed(PathBuf, String),

    #[error("vm config already exist {0:?}")]
    VMConfigAlreadyExist(PathBuf),

    #[error("vm disk error {0:?}")]
    DiskError(DiskError),

    #[error("vm run command name {0} with error {1:?}")]
    CmdError(String,#[source] std::io::Error),

    #[error("vm start failed with name {0} with error {1:?}")]
    QemuStartFailed(String, Option<i32>),

    #[error("vm qmp client name {0} with error {1:?}")]
    QmpClient(String,#[source] std::io::Error),
}

#[derive(Error, Debug)]
pub enum QemuError {
    
}
