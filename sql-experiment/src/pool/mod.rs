use crate::pool::common::Connection;

pub mod common;
#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

pub enum Pool {
    #[cfg(feature = "mssql")]
    Mssql,
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "sqlite")]
    Sqlite(deadpool_sqlite::Pool),
}

impl Pool {
    async fn get_connection(&self) -> Result<impl Connection, String> {
        match self {
            Pool::Mssql => {
                todo!()
            }
            Pool::Postgres => {
                todo!()
            }
            Pool::Sqlite(pool) => {
                let conn = pool.get().await.map_err(|e| e.to_string())?;
                Ok(sqlite::SqliteConnection { conn })
            }
        }
    }
}
