#[derive(thiserror::Error, Debug)]
pub enum PostgresError {

    #[error(transparent)]
    PoolBuildError(#[from] deadpool_postgres::BuildError),

    #[error(transparent)]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("{0}")]
    Other(String),
}
