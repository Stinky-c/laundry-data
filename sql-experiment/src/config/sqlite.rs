use crate::config::traits::ToPool;
use crate::error::pool::PoolBuilderError;
use crate::pool::common::Pool;
use crate::pool::sqlite::SqlitePool;
use std::path::PathBuf;

pub struct SqliteConfig {
    inner: deadpool_sqlite::Manager,
}

#[bon::bon]
impl SqliteConfig {
    #[builder(on(String, into))]
    pub fn builder(#[builder(into)] path: PathBuf) -> Self {
        let config = deadpool_sqlite::Config::new(path);
        let manager =
            deadpool_sqlite::Manager::from_config(&config, deadpool_sqlite::Runtime::Tokio1);
        Self { inner: manager }
    }
}

impl ToPool for SqliteConfig {
    fn to_pool(self) -> Result<Pool, PoolBuilderError> {
        let inner = deadpool_sqlite::Pool::builder(self.inner).build()?;
        Ok(Pool::Sqlite(SqlitePool::new(inner)))
    }
}
