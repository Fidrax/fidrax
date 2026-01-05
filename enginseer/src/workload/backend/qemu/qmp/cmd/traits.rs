use serde::Serialize;
use serde_json::Value;

/// QmpExecute
#[derive(Serialize)]
pub struct QmpExecute<'a> {
    pub execute: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}

// QmpCommand Trait
pub trait QmpCmdRequest {
    /// The Qmp commands name ("query-status", "stop", etc)
    fn cmd_name(&self) -> &'static str;

    /// Convert this command into a serializable arguments object
    fn to_args(&self) -> Option<Value>;

    /// Build a QmpExecute struct from this command
    fn to_execute(&self, id: Option<u64>) -> QmpExecute<'static> {
        QmpExecute {
            execute: self.cmd_name(),
            arguments: self.to_args(),
            id,
        }
    }
    type Response: serde::de::DeserializeOwned;
}
