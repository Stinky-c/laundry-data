#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;
pub mod traits;

pub enum PoolConnection {
    #[cfg(feature = "mssql")]
    Mssql(mssql::TokioClient),
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "sqlite")]
    Sqlite(async_sqlite::Pool),
}
