use crate::config::traits::ToPool;
use crate::pool::common::Pool;
use crate::pool::mssql::MsSqlPool;
use tiberius::AuthMethod;

pub struct MssqlConfig {
    inner: deadpool_tiberius::Manager,
}

#[bon::bon]
impl MssqlConfig {
    #[builder(on(String, into))]
    pub fn builder(
        #[builder(default = MssqlConfig::default_host())] host: String,
        #[builder(default = MssqlConfig::default_port())] port: u16,
        auth_method: AuthMethod,
        #[builder(default = MssqlConfig::default_database())] database: String,
        instance_name: Option<String>,
        application_name: Option<String>,
        #[builder(default = MssqlConfig::default_encryption())]
        encryption: tiberius::EncryptionLevel,
    ) -> Self {
        let mut manager = deadpool_tiberius::Manager::new()
            .host(host)
            .port(port)
            .authentication(auth_method)
            .encryption(encryption);

        if let Some(instance_name) = instance_name {
            manager = manager.instance_name(instance_name);
        }

        if let Some(application_name) = application_name {
            manager = manager.application_name(application_name);
        }

        Self { inner: manager }
    }
}

// Default fields
impl MssqlConfig {
    /// Get the default host
    ///
    /// - Defaults to: `localhost`
    fn default_host() -> String {
        "localhost".to_string()
    }

    /// Get the default port
    ///
    /// - Defaults to: `1433`
    fn default_port() -> u16 {
        1433
    }

    /// Get the default database
    ///
    /// - Defaults to: `master`
    fn default_database() -> String {
        "master".to_string()
    }

    /// Get the default encryption support for SQL Server
    ///
    /// - Defaults to: [`tiberius::EncryptionLevel::On`]
    fn default_encryption() -> tiberius::EncryptionLevel {
        tiberius::EncryptionLevel::On
    }
}

impl ToPool for MssqlConfig {
    type Error = crate::error::MsSqlError;
    fn to_pool(self) -> Result<Pool, Self::Error> {
        let inner = deadpool_tiberius::Pool::builder(self.inner).build()?;
        Ok(Pool::MsSql(MsSqlPool::new(inner)))
    }
}
