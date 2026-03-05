use crate::connection::sqlite::SqliteConnection;
use crate::connection::common::Connection;
use crate::pool::common::ToConnection;
use async_trait::async_trait;
use crate::error::pool::CommonPoolError;

pub struct SqlitePool {
    inner: deadpool_sqlite::Pool,
}

impl SqlitePool {
    pub(crate) fn new(inner: deadpool_sqlite::Pool) -> Self {
        Self { inner }
    }
}

impl From<deadpool_sqlite::Pool> for SqlitePool {
    fn from(value: deadpool_sqlite::Pool) -> Self {
        Self { inner: value }
    }
}

#[async_trait]
impl ToConnection for SqlitePool {
    async fn get_connection(&self) -> Result<Connection, CommonPoolError> {
        let conn = self.inner.get().await?;
        Ok(Connection::Sqlite(SqliteConnection::new(conn)))
    }
}
