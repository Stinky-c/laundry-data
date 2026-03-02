#[cfg(feature = "mssql")]
pub mod mssql;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;
pub mod traits;

pub enum ConfigType {
    #[cfg(feature = "mssql")]
    Mssql(mssql::MssqlConfig),
    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresConfig),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqliteConfig),
}

// TODO: add ssl support
