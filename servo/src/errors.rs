use enginseer::errors::EnginseerErrors;
use thiserror::Error;

use crate::api::dtos::errors::DTOSErrors;

#[derive(Error, Debug)]
pub enum ServoErrors {
    #[error("enginseer error ")]
    EnginseerErrors(EnginseerErrors),

    #[error("dtos error")]
    DTOS(DTOSErrors),
}
