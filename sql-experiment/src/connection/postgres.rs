pub struct PostgresConnection {
    inner: deadpool_postgres::Object,
}

impl PostgresConnection {
    pub(crate) fn new(inner: deadpool_postgres::Object) -> Self {
        Self { inner }
    }
}
