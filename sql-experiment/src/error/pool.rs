use deadpool::managed::{BuildError, PoolError};

/// An error case for when attempting to get an object from the pool.
/// Commonly a timeout or a backend error
#[derive(thiserror::Error, Debug)]
pub enum CommonPoolError {
    #[error("No runtime specified")]
    NoRuntimeSpecified,

    #[error("Pool timed out on '{0:?}'")]
    PoolTimeout(deadpool::managed::TimeoutType),

    #[error("{msg}")]
    PoolBackendError { msg: String },

    #[error("Pool is closed")]
    PoolClosed,

    #[error("{msg}")]
    PoolPostCreateHookError { msg: String },
}

impl<C> From<PoolError<C>> for CommonPoolError
where
    C: core::error::Error,
{
    fn from(value: PoolError<C>) -> Self {
        match value {
            PoolError::Timeout(t) => Self::PoolTimeout(t),
            PoolError::Backend(err) => Self::PoolBackendError {
                msg: err.to_string(),
            },
            PoolError::Closed => Self::PoolClosed,
            PoolError::NoRuntimeSpecified => Self::NoRuntimeSpecified,
            PoolError::PostCreateHook(e) => Self::PoolPostCreateHookError { msg: e.to_string() },
        }
    }
}

/// Error for when building a pool. Currently only has a single case that should never happen
#[derive(thiserror::Error, Debug)]
pub enum PoolBuilderError {
    #[error("No runtime specified")]
    NoRuntimeSpecified,
}
impl From<BuildError> for PoolBuilderError {
    fn from(err: BuildError) -> Self {
        match err {
            BuildError::NoRuntimeSpecified => Self::NoRuntimeSpecified,
        }
    }
}
