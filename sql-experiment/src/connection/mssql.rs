use crate::connection::common::{Execute, Query};

pub struct MsSqlConnection {
    inner: deadpool::managed::Object<deadpool_tiberius::Manager>,
}
impl MsSqlConnection {
    pub(crate) fn new(inner: deadpool::managed::Object<deadpool_tiberius::Manager>) -> Self {
        Self { inner }
    }
}

impl Query for MsSqlConnection {}
impl Execute for MsSqlConnection {}
