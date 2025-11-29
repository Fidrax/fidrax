use serde::Serialize;

use crate::vm::backend::qemu::qmp::cmd::{commands::QmpCmdName, empty::EmptyResponse, traits::QmpCmdRequest};


#[derive(Debug, Serialize)]
pub struct SystemPowerdown;

impl QmpCmdRequest for SystemPowerdown{
    fn cmd_name(&self) -> &'static str {
        QmpCmdName::SystemPowerdown.as_str()
    }

    fn to_args(&self) -> Option<serde_json::Value> {
        None
    }

    type Response = EmptyResponse;
}