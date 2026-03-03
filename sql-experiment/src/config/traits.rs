use async_trait::async_trait;

#[async_trait]
/// A handler for configs to be converted into their respective deadpool managed pool.
pub trait ToPool {
    type Error;
    async fn to_pool(&self) -> Result<(), Self::Error>;
}
