use enginseer::errors::EnginseerErrors;
use thiserror::Error;

use crate::api::dtos::errors::DTOSErrors;

#[derive(Error, Debug)]
pub enum ServoErrors {
    #[error("enginseer error {0}")]
    EnginseerErrors(#[from] EnginseerErrors),

    #[error("dtos error {0}")]
    DTOS(#[from] DTOSErrors),
}
