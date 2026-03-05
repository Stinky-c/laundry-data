use crate::connection::mssql::MsSqlConnection;
use crate::connection::common::Connection;
use crate::error::pool::CommonPoolError;
use crate::pool::common::ToConnection;
use async_trait::async_trait;

pub struct MsSqlPool {
    inner: deadpool_tiberius::Pool,
}
impl MsSqlPool {
    pub(crate) fn new(inner: deadpool_tiberius::Pool) -> Self {
        Self { inner }
    }
}

impl From<deadpool_tiberius::Pool> for MsSqlPool {
    fn from(value: deadpool_tiberius::Pool) -> Self {
        Self { inner: value }
    }
}

#[async_trait]
impl ToConnection for MsSqlPool {
    async fn get_connection(&self) -> Result<Connection, CommonPoolError> {
        let conn = self.inner.get().await?;
        Ok(Connection::MsSql(MsSqlConnection::new(conn)))
    }
}
