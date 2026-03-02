use crate::config::traits::ToConnectionPool;
use crate::error::PostgresError;
use crate::pool::PoolConnection;
use async_trait::async_trait;
use tokio_postgres::Config as TpConfig;

#[derive(bon::Builder)]
#[builder(on(String, into))]
pub struct PostgresConfig {
    #[builder(default = PostgresConfig::default_host())]
    pub(crate) host: String,
    #[builder(default = PostgresConfig::default_port())]
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    #[builder(default = PostgresConfig::default_database())]
    pub(crate) database: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
            user: Self::default_username(),
            password: Self::default_password(),
            database: Self::default_database(),
        }
    }
}

// Default fields
impl PostgresConfig {
    /// Get the default host
    ///
    /// - Defaults to: `localhost`
    fn default_host() -> String {
        "localhost".to_string()
    }

    /// Get the default username
    ///
    /// - Defaults to: `postgres`
    fn default_username() -> String {
        "postgres".to_string()
    }

    /// Get the default password
    ///
    /// - Defaults to: `postgres`
    fn default_password() -> String {
        "postgres".to_string()
    }

    /// Get the default port
    ///
    /// - Defaults to: `5432`
    fn default_port() -> u16 {
        5432
    }

    /// Get the default database
    ///
    /// - Defaults to: `master`
    fn default_database() -> String {
        "postgres".to_string()
    }
}

impl Into<TpConfig> for PostgresConfig {
    fn into(self) -> TpConfig {
        let conf = TpConfig::new()
            .host(self.host)
            .port(self.port)
            .user(self.user)
            .password(self.password)
            .dbname(self.database)
            .to_owned();

        conf
    }
}

#[async_trait]
impl ToConnectionPool for PostgresConfig {
    type Error = PostgresError;

    async fn init_connection(self) -> Result<PoolConnection, Self::Error> {
        todo!()
    }
}
