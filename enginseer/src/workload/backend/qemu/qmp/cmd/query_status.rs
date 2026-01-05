use serde::{Deserialize, Serialize};

use crate::workload::backend::qemu::qmp::cmd::{commands::QmpCmdName, traits::QmpCmdRequest};


// request
#[derive(Serialize, Debug)]
pub struct QueryStatusRequest;

impl QmpCmdRequest for QueryStatusRequest {
    fn cmd_name(&self) -> &'static str {
        QmpCmdName::QueryStatus.as_str()
    }

    fn to_args(&self) -> Option<serde_json::Value> {
        None
    }

    type Response = QueryStatusResponse;
}

// response
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum QemuStatus {
    Running,
    Paused,
    Shutdown,
    Inmigrate,
    Prelaunch,
    FinishMigrate,
    RestoreVm,
    Watchdog,
    GuestPanicked,
    Postmigrate,
    Cold,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryStatusResponse {
    pub status: QemuStatus,
    pub singlestep: Option<bool>,
    pub running: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMsQueryStatusResponse {
    pub name: String,
    pub qmp_path: String,
    pub state: QueryStatusResponse,
}
