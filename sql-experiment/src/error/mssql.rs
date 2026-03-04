use tiberius::error::Error as TiberiusError;
#[derive(thiserror::Error, Debug)]
pub enum MsSqlError {

    #[error(transparent)]
    PoolBuildError(#[from] deadpool::managed::BuildError),

    #[error(transparent)]
    PoolError(#[from] deadpool::managed::PoolError<TiberiusError>),

    #[error("{0}")]
    Other(String),
}
