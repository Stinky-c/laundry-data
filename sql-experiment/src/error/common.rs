use crate::error::pool::{CommonPoolError, PoolBuilderError};
use crate::error::query::QueryError;
use deadpool::managed::CreatePoolError;

// TODO: find a way to have backtraces + sources
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum CommonError {
    #[error("Missing runtime configuration")]
    MissingRuntime,

    #[error("{msg}")]
    CreatePoolConfigError { msg: String },

    #[error(transparent)]
    PoolError(#[from] CommonPoolError),

    #[error(transparent)]
    PoolBuildError(#[from] PoolBuilderError),

    #[error(transparent)]
    QueryError(#[from] QueryError),
}

impl<C> From<CreatePoolError<C>> for CommonError
where
    C: core::error::Error,
{
    fn from(value: CreatePoolError<C>) -> Self {
        match value {
            CreatePoolError::Config(err) => CommonError::CreatePoolConfigError {
                msg: err.to_string(),
            },
            CreatePoolError::Build(_) => CommonError::MissingRuntime,
        }
    }
}
