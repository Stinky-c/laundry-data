pub struct SqliteConnection {
    inner: deadpool_sqlite::Object,
}

impl SqliteConnection {
    pub(crate) fn new(inner: deadpool_sqlite::Object) -> Self {
        Self { inner }
    }
}
