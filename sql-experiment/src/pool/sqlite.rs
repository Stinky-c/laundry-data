use crate::pool::common::ToConnection;

pub struct SqlitePool {
    inner: deadpool_sqlite::Pool,
}

impl SqlitePool {
    pub(crate) fn new(inner: deadpool_sqlite::Pool) -> Self {
        Self { inner }
    }
}

impl From<deadpool_sqlite::Pool> for SqlitePool {
    fn from(value: deadpool_sqlite::Pool) -> Self {
        Self { inner: value }
    }
}

impl ToConnection for SqlitePool {}
