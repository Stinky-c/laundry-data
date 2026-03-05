#[cfg(feature = "mssql")]
pub mod mssql;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub enum Connection {
    #[cfg(feature = "mssql")]
    MsSql(mssql::MssqlConnection),
    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresConnection),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqliteConnection),
}
