use serde::Deserialize;

use std::fmt::{Debug, Formatter};
refinery::embed_migrations!("migrations");

#[derive(Clone, Deserialize, Eq, PartialEq)]
pub(crate) struct DbConfig {
    /// Hostname or ip to connect to.
    /// Env: `DB_HOST`
    pub(crate) host: String,
    /// Port to connect to.
    /// Env: `DB_PORT`
    pub(crate) port: u16,
    /// Database to connect to.
    /// Env: `DB_NAME`
    #[serde(alias = "name")]
    pub(crate) db_name: String,
    /// Database Username.
    /// Env: `DB_USER`
    #[serde(alias = "user")]
    pub(crate) user_name: String,
    /// Database password.
    /// Env: `DB_PASS`
    #[serde(alias = "pass")]
    pub(crate) password: String,
}

// Hide password from debug
impl Debug for DbConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("db_name", &self.db_name)
            .field("username", &self.user_name)
            .finish()
    }
}

/*
DB:
Try to insert, if failing send a message to a once of thread to get extra data
https://docs.rs/tokio/latest/tokio/sync/index.html#mpsc-channel
https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html
 */
