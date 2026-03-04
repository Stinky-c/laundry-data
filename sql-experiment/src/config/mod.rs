use crate::DatabaseType;

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

/// A single object with all possible fields for all databases.
/// Conversion is handled by a trait and the type is determined by the mandatory
///
#[cfg_attr(feature = "config_builder", derive(bon::Builder))]
pub struct CommonDatabaseConfig {
    //TODO add serde + builder support
    r#type: DatabaseType,
}
