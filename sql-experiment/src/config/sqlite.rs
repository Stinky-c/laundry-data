use std::path::PathBuf;

#[derive(bon::Builder)]
#[builder(on(String, into))]
pub struct SqliteConfig {
    #[builder(into)]
    pub(crate) path: PathBuf,
    #[builder(default)]
    pub(crate) open_flags: rusqlite::OpenFlags,
    #[builder(default = 4)]
    pub(crate) max_connections: usize,
}

// Cannot impl default. Needs path

use crate::config::traits::ToPool;
use crate::error::SqlitePoolError;
use async_trait::async_trait;

#[async_trait]
impl ToPool for SqliteConfig {
    type Error = SqlitePoolError;

    async fn to_pool(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
