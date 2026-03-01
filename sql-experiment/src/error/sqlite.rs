use async_sqlite::rusqlite;

#[derive(thiserror::Error, Debug)]
pub enum SqlitePoolError {
    #[error(transparent)]
    AsyncSqlite(#[from] async_sqlite::Error),
}
