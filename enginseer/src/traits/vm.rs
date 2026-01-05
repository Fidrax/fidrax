use crate::workload::errors::VMError;

pub trait VirtualMachine<T> {
    async fn start(&self, config: T) -> Result <(), VMError>;
    async fn stop(&self) -> Result<(), VMError>;
    async fn pause(&self) -> Result<(), VMError>;
    async fn restart(&self) -> Result<(), VMError>;
    async fn status(&self) -> Result<(), VMError>;
}