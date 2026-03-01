use crate::config::sqlite::SqliteConfig;
use crate::error::SqlitePoolError;
use crate::pool::PoolConnection;
use crate::prelude::ToConnectionPool;
use async_trait::async_trait;

#[async_trait]
impl ToConnectionPool for SqliteConfig {
    type Error = SqlitePoolError;

    async fn init_connection(self) -> Result<PoolConnection, Self::Error> {
        let builder = self.into_pool_builder();
        let pool = builder.open().await.map_err(SqlitePoolError::AsyncSqlite)?;

        Ok(PoolConnection::Sqlite(pool))
    }
}
