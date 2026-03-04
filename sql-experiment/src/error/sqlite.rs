#[derive(thiserror::Error, Debug)]
pub enum SqliteError {

    #[error(transparent)]
    PoolBuildError(#[from] deadpool_sqlite::BuildError),

    #[error(transparent)]
    PoolError(#[from] deadpool_sqlite::PoolError),

    #[error("{0}")]
    Other(String),
}
