pub mod config;
pub mod error;
pub mod pool;
mod query;
#[cfg(test)]
mod tests;

/// This enum denotes all supported databases.
/// Does not note all database support that is compiled in.
pub enum DatabaseType {
    Mssql,
    Postgres,
    Sqlite,
}

pub mod prelude {
    pub use crate::pool::PoolConnection;
    pub use crate::pool::traits::ToConnectionPool;
}

#[cfg(feature = "mssql")]
pub mod mssql_prelude {
    pub use tiberius::AuthMethod;

    pub use crate::config::mssql::MssqlConfig;
    pub use crate::error::MsSqlError;
    pub use crate::prelude::*;
}

#[cfg(feature = "postgres")]
pub mod postgres_prelude {
    pub use crate::config::postgres::PostgresConfig;
    pub use crate::prelude::*;
}

#[cfg(feature = "sqlite")]
pub mod sqlite_prelude {
    pub use crate::config::sqlite::SqliteConfig;
    pub use crate::error::SqlitePoolError;
    pub use crate::prelude::*;
}
