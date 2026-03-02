use async_trait::async_trait;

#[async_trait]
pub trait ToConnectionPool {
    type Error;
    async fn init_connection(self) -> Result<crate::pool::PoolConnection, Self::Error>;
}