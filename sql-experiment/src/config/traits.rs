use async_trait::async_trait;

#[async_trait]
pub trait ToPool {
    type Error;
    async fn to_pool(&self) -> Result<(), Self::Error>;
}
