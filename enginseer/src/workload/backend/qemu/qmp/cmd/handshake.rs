use crate::workload::backend::qemu::qmp::cmd::commands::QmpCmdName;

use super::traits::QmpCmdRequest;
use serde::{Deserialize, Serialize};

// request
#[derive(Serialize, Debug)]
pub struct ProtocolHandshakeRequest;

impl QmpCmdRequest for ProtocolHandshakeRequest {
    fn cmd_name(&self) -> &'static str {
        QmpCmdName::as_str(&QmpCmdName::QmpCapabilities)
    }

    fn to_args(&self) -> Option<serde_json::Value> {
        None
    }

    type Response = ProtocolHandshakeResponse;
}

// response
#[derive(Deserialize, Debug)]
pub struct Version {
    pub qemu: String,
}

#[derive(Deserialize, Debug)]
pub struct ProtocolHandshakeResponse {
    pub version: Option<Version>,
    pub capabilities: Option<Vec<String>>,
}
