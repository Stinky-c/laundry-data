use crate::pool::common::Connection;

pub struct SqliteConnection {
    pub(crate) conn: deadpool_sqlite::Connection,
}

impl Connection for SqliteConnection {}
