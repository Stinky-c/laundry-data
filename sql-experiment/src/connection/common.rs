use crate::connection::{mssql, postgres, sqlite};
use crate::error::query::QueryError;
use ambassador::{delegatable_trait, Delegate};
use async_trait::async_trait;

#[derive(Delegate)]
#[delegate(Query)]
#[delegate(Execute)]
pub enum Connection {
    #[cfg(feature = "mssql")]
    MsSql(mssql::MsSqlConnection),
    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresConnection),
    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqliteConnection),
}

#[async_trait]
#[delegatable_trait]
pub trait Query {
    async fn query<R>(&self, sql: String) -> Result<R, QueryError> {
        todo!()
    }
}

#[async_trait]
#[delegatable_trait]
pub trait Execute {
    // ExecutableError
    async fn execute<R>(&self, sql: String) -> Result<R, String> {
        todo!()
    }
}
