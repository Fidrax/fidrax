#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState{
    Stopped,
    Running,
    Paused,
    Error,
    Unknown,
}