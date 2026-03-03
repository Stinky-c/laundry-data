#[derive(thiserror::Error, Debug)]
pub enum SqlitePoolError {
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
}
