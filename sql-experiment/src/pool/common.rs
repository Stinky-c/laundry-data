use crate::connection::common::Connection;
use crate::error::pool::CommonPoolError;
use ambassador::{delegatable_trait, Delegate};
use async_trait::async_trait;

/// A blanket implementation defining the ability to get a connection
///
#[async_trait]
#[delegatable_trait]
pub trait ToConnection {
    /// Get a connection object from the pool
    async fn get_connection(&self) -> Result<Connection, CommonPoolError>;
}

#[derive(Delegate)]
#[delegate(ToConnection)]
pub enum Pool {
    #[cfg(feature = "mssql")]
    MsSql(crate::pool::mssql::MsSqlPool),
    #[cfg(feature = "postgres")]
    Postgres(crate::pool::postgres::PostgresPool),
    #[cfg(feature = "sqlite")]
    Sqlite(crate::pool::sqlite::SqlitePool),
}
