use crate::connection::common::{Execute, Query};
use async_trait::async_trait;

pub struct SqliteConnection {
    inner: deadpool_sqlite::Object,
}

impl SqliteConnection {
    pub(crate) fn new(inner: deadpool_sqlite::Object) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl Query for SqliteConnection {}
impl Execute for SqliteConnection {}