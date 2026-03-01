use async_sqlite::{JournalMode, PoolBuilder};
use std::path::PathBuf;

#[derive(bon::Builder)]
#[builder(on(String, into))]
pub struct SqliteConfig {
    #[builder(into)]
    pub(crate) path: PathBuf,
    #[builder(default)]
    pub(crate) open_flags: async_sqlite::rusqlite::OpenFlags,
    #[builder(default = 2)]
    pub(crate) max_connections: usize,
    pub(crate) journal_mode: Option<JournalMode>,
}

// Cannot impl default. Needs path

impl SqliteConfig {
    /// explicit conversion into [`async_sqlite::PoolBuilder`]
    pub(crate) fn into_pool_builder(self) -> PoolBuilder {
        self.into()
    }
}

impl Into<PoolBuilder> for SqliteConfig {
    fn into(self) -> PoolBuilder {
        let mut conf = PoolBuilder::new()
            .path(self.path)
            .flags(self.open_flags)
            .num_conns(self.max_connections);

        if let Some(mode) = self.journal_mode {
            conf = conf.journal_mode(mode)
        }

        conf
    }
}
