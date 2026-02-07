#[cfg(all(feature = "postgres", feature = "mssql"))]
compile_error!("feature \"postgres\" and feature \"mssql\" cannot be enabled at the same time");

pub(crate) mod embedded {
    use refinery::embed_migrations;
    use refinery::{Error, Report,};
    use sql_middleware::MiddlewarePoolConnection;
    use crate::db::wrap;

    embed_migrations!("migrations");

    pub(crate) async fn run_async(conn: MiddlewarePoolConnection) -> Result<Report, Error> {
        let runner = migrations::runner();
        runner.run_async(&mut wrap(conn)).await
    }
}

// NOTE: Maybe break out into separate crate.
use refinery::{AsyncMigrate, Migration};
use refinery_core::traits::r#async::{AsyncQuery, AsyncTransaction};
use sql_middleware::{CustomDbRow, MiddlewarePoolConnection, SqlMiddlewareDbError};

/// Wrap MiddlewarePoolConnection
struct MiddlewarePoolConnectionWrapper(MiddlewarePoolConnection);

fn migration_from_row(row: CustomDbRow) -> Result<Migration, SqlMiddlewareDbError> {
    let version: i32 = {
        let row = match row.get_by_index(0) {
            Some(row) => Ok(row),
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get index 0".to_string(),
            )),
        }?;

        match row.as_int() {
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get value".to_string(),
            )),
            Some(value) => Ok(value.to_owned() as i32),
        }?
    };
    let name: String = {
        let row = match row.get_by_index(1) {
            Some(row) => Ok(row),
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get index 1".to_string(),
            )),
        }?;

        match row.as_text() {
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get value".to_string(),
            )),
            Some(value) => Ok(value.to_owned()),
        }?
    };
    let applied_on: time::OffsetDateTime = {
        let row = match row.get_by_index(2) {
            Some(row) => Ok(row),
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get index 2".to_string(),
            )),
        }?;

        match row.as_timestamp() {
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get value".to_string(),
            )),
            // TODO: ensure this has the right timestamp
            Some(value) => time::OffsetDateTime::from_unix_timestamp(value.and_utc().timestamp())
                .map_err(|_| {
                    SqlMiddlewareDbError::Unimplemented("Failed to convert value".to_string())
                }),
        }?
    };
    let checksum: u64 = {
        let row = match row.get_by_index(3) {
            Some(row) => Ok(row),
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get index 0".to_string(),
            )),
        }?;

        match row.as_int() {
            None => Err(SqlMiddlewareDbError::Unimplemented(
                "Failed to get value".to_string(),
            )),
            Some(value) => Ok(value.to_owned() as u64),
        }?
    };
    Ok(Migration::applied(version, name, applied_on, checksum))
}

// An async query that expects to return a vec of migrations
impl AsyncQuery<Vec<Migration>> for MiddlewarePoolConnectionWrapper {
    async fn query(&mut self, query: &str) -> Result<Vec<Migration>, Self::Error> {
        // TODO: impl starting a transaction
        let value = self.0.query(query).select().await?;
        let mut migrations: Vec<Migration> = vec![];

        for row in value.results {
            let migration = migration_from_row(row)?;
            migrations.push(migration);
        }

        Ok(migrations)
    }
}

// Begin a transaction and run queries
impl AsyncTransaction for MiddlewarePoolConnectionWrapper {
    type Error = SqlMiddlewareDbError;

    async fn execute<'a, T: Iterator<Item = &'a str> + Send>(
        &mut self,
        queries: T,
    ) -> Result<usize, Self::Error> {
        match self.0 {
            #[cfg(feature = "mssql")]
            MiddlewarePoolConnection::Mssql { conn, .. } => {
                let mut tx = sql_middleware::mssql::begin_transaction(&mut *conn).await?;
                let mut count = 0;
                for i in queries {
                    tx.execute_batch(i).await?;
                    count += 1;
                }

                let _ = tx.commit().await?;
                Ok(count as usize)
            }
            #[cfg(feature = "postgres")]
            MiddlewarePoolConnection::Postgres { client, .. } => {
                let mut tx = sql_middleware::postgres::begin_transaction(client).await?;
                let mut count = 0;
                for i in queries {
                    tx.execute_batch(i).await?;
                    count += 1;
                }
                let _ = tx.commit().await?;
                Ok(count as usize)
            }
            _ => unimplemented!("Too lazy to add."),
        }
    }
}

impl AsyncMigrate for MiddlewarePoolConnectionWrapper {}

impl std::ops::Deref for MiddlewarePoolConnectionWrapper {
    type Target = MiddlewarePoolConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrap MiddlewarePoolConnection into a type for handling
pub fn wrap(conn: MiddlewarePoolConnection) -> MiddlewarePoolConnectionWrapper {
    MiddlewarePoolConnectionWrapper(conn)
}


