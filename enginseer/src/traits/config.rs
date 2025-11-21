pub trait Config<T, E> {
    async fn read(&self, name: &str) -> Result<T, E>;
    async fn create(&self, config: &T) -> Result<(), E>;
    async fn update(&self, config: &T) -> Result<(), E>;
    async fn delete(&self, name: &str) -> Result<(), E>;
    async fn list(&self) -> Result<Vec<T>, E>;
    fn validate(&self, config: &T) -> Result<(), E>;
}