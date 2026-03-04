use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

/// A blanket implementation defining the ability to get a connection
#[enum_dispatch(Pool)]
#[async_trait]
pub trait ToConnection {
    /// Get a connection object from the pool
    fn get_connection(&self) -> Connection {
        todo!()
    }
}

#[enum_dispatch]
pub enum Pool {
    #[cfg(feature = "mssql")]
    MsSql(crate::pool::mssql::MsSqlPool),
    #[cfg(feature = "postgres")]
    Postgres(crate::pool::postgres::PostgresPool),
    #[cfg(feature = "sqlite")]
    Sqlite(crate::pool::sqlite::SqlitePool),
}

pub enum Connection {
    #[cfg(feature = "mssql")]
    MsSql,
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "sqlite")]
    Sqlite,
}
