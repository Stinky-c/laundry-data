use async_trait::async_trait;

/// A handler for configs to be converted into their respective deadpool managed pool.
pub trait ToPool {
    type Error;
    fn to_pool(self) -> Result<crate::pool::common::Pool, Self::Error>;
}
