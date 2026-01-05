use crate::{storage, workload::errors::VMError};

use storage::errors::DiskError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnginseerErrors {
    #[error("disk error: {0}")]
    Disk(#[from] DiskError),
    
    #[error("vm error: {0}")]
    VM(#[from] VMError),
}
