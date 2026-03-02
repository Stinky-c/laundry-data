#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

pub enum PoolConnection {
    #[cfg(feature = "mssql")]
    Mssql(crate::config::mssql::TokioClient),
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "sqlite")]
    Sqlite(async_sqlite::Pool),
}
