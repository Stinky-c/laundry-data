pub struct MssqlConnection {
    inner: deadpool::managed::Object<deadpool_tiberius::Manager>,
}
impl MssqlConnection {
    pub(crate) fn new(inner: deadpool::managed::Object<deadpool_tiberius::Manager>) -> Self {
        Self { inner }
    }
}
