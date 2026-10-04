use serde::Deserialize;

use std::fmt::{Debug, Formatter};
refinery::embed_migrations!("migrations");

/// Type of Database. One of `postgres`, `pgsql`, `sqlite`, `mssql`.
static ENV_DB_TYPE: &str = "DB_TYPE";
/// Hostname or ip to connect to.
static ENV_DB_HOST: &str = "DB_HOST";
/// Port to connect to.
static ENV_DB_PORT: &str = "DB_PORT";
/// Database to connect to.
static ENV_DB_NAME: &str = "DB_NAME";
/// Database Username.
static ENV_DB_USER: &str = "DB_USER";
/// Database password.
static ENV_DB_PASS: &str = "DB_PASS";
/// Database file path. Used for sqlite.
static ENV_DB_PATH: &str = "DB_PATH";

#[derive(Clone, Deserialize, Eq, PartialEq)]
pub(crate) struct DbConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    #[serde(alias = "name")]
    pub(crate) db_name: String,
    #[serde(alias = "user")]
    pub(crate) user_name: String,
    #[serde(alias = "pass")]
    pub(crate) password: String,

}

impl DbConfig {
    fn default_tz() -> String {
        "UTC".into()
    }
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
