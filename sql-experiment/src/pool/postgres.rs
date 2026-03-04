use crate::pool::common::ToConnection;

pub struct PostgresPool {
    inner: deadpool_postgres::Pool,
}

impl PostgresPool {
    pub(crate) fn new(inner: deadpool_postgres::Pool) -> Self {
        Self { inner }
    }
}

impl From<deadpool_postgres::Pool> for PostgresPool {
    fn from(value: deadpool_postgres::Pool) -> Self {
        Self { inner: value }
    }
}

impl ToConnection for PostgresPool {}
