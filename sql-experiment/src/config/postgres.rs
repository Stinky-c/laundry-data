use crate::config::traits::ToPool;
use crate::pool::common::Pool;
use crate::pool::postgres::PostgresPool;
use tokio_postgres::NoTls;

pub struct PostgresConfig {
    inner: deadpool_postgres::Manager,
}

#[bon::bon]
impl PostgresConfig {
    #[builder(on(String, into))]
    pub fn builder(
        #[builder(default = PostgresConfig::default_host())] host: String,
        #[builder(default = PostgresConfig::default_port())] port: u16,
        user: String,
        password: String,
        #[builder(default = PostgresConfig::default_database())] database: String,
    ) -> Self {
        let config = tokio_postgres::Config::new()
            .host(host)
            .port(port)
            .user(user)
            .password(password)
            .to_owned();
        let manager = deadpool_postgres::Manager::new(config, NoTls);

        Self { inner: manager }
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

impl ToPool for PostgresConfig {
    type Error = crate::error::PostgresError;
    fn to_pool(self) -> Result<Pool, Self::Error> {
        let inner = deadpool_postgres::Pool::builder(self.inner).build()?;
        Ok(Pool::Postgres(PostgresPool::new(inner)))
    }
}
