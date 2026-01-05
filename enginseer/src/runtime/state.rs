use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RuntimeState {
    // unkown
    Unknown,
    // exist as config
    Defined,
    // runtime booting
    Starting,
    // active running
    Running,
    // paused
    Paused,
    // gracefully stopping
    Stopping,
    // stopped cleanly
    Stopped,
    // exited
    Exited,
    // shutdown
    Shutdown,
    // crash or failed
    Failed { reason: String },
}
