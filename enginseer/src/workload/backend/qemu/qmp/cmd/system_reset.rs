use serde::Serialize;

use crate::workload::backend::qemu::qmp::cmd::{commands::QmpCmdName, empty::EmptyResponse, traits::QmpCmdRequest};


#[derive(Debug, Serialize)]
pub struct SystemReset;

impl QmpCmdRequest for SystemReset {
    fn cmd_name(&self) -> &'static str {
        QmpCmdName::SystemReset.as_str()
    }

    fn to_args(&self) -> Option<serde_json::Value> {
        None
    }

    type Response = EmptyResponse;
}