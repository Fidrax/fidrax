use enginseer::disk::errors::DiskError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DTOSErrors {
    #[error("enginseer disk error ")]
    Disk(DiskError),

    #[error("disk create request name is empty {0}")]
    DiskCreateRequestNameEmpty(String),

    #[error("disk create request name contain invalid char {0}")]
    DiskCreateRequestInvalidName(String),

    #[error("disk create request invalid size {0}")]
    DiskSizeInvalid(String),

    #[error("disk create request disk path is empty {0}")]
    DiskPathEmpty(String),

    #[error("disk create request disk path should start with / {0}")]
    DiskPathStartPathInvalid(String),

    #[error("disk create request disk alloc mode invalid {0}")]
    DiskInvalidAllocMode(String),

    #[error("vm create request name is empty {0}")]
    VMCreateRequestNameEmpty(String),

    #[error("vm create request name contain invalid char {0}")]
    VMCreateRequestInvalidName(String),

    #[error("vm memory request invalid size {0}")]
    VMMemSizeInvalid(String),

    #[error("vm vcpu request invalid size {0}")]
    VMCpuSizeInvalid(String),
}
