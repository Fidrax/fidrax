pub enum QmpCmdName {
    QmpCapabilities,
    QueryStatus,
    Stop,
    SystemPowerdown,
    SystemReset,
}

impl QmpCmdName {
    pub fn as_str(&self) -> &'static str {
        match self {
            QmpCmdName::QmpCapabilities => "qmp_capabilities",
            QmpCmdName::QueryStatus => "query-status",
            QmpCmdName::Stop => "stop",
            QmpCmdName::SystemPowerdown => "system_powerdown",
            QmpCmdName::SystemReset => "system_reset",
        }
    }
}
