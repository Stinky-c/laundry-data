use crate::error::pool::PoolBuilderError;

/// A handler for configs to be converted into their respective deadpool managed pool.
pub trait ToPool {
    fn to_pool(self) -> Result<crate::pool::common::Pool, PoolBuilderError>;
}
