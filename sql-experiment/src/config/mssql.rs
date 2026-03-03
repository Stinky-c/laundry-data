use tiberius::{AuthMethod, Config as TbConfig};

#[derive(bon::Builder)]
#[builder(on(String, into))]
pub struct MssqlConfig {
    #[builder(default = MssqlConfig::default_host())]
    pub(crate) host: String,
    #[builder(default = MssqlConfig::default_port())]
    pub(crate) port: u16,
    pub(crate) auth_method: AuthMethod,
    #[builder(default = MssqlConfig::default_database())]
    pub(crate) database: String,
    pub(crate) instance_name: Option<String>,
    pub(crate) application_name: Option<String>,
    #[builder(default = MssqlConfig::default_encryption())]
    pub(crate) encryption: tiberius::EncryptionLevel,
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

impl MssqlConfig {
    pub(crate) fn into_tiberius_config(self) -> TbConfig {
        self.into()
    }
}

impl Into<TbConfig> for MssqlConfig {
    fn into(self) -> TbConfig {
        let mut conf = TbConfig::new();

        conf.host(self.host);
        conf.port(self.port);
        conf.database(self.database);
        conf.encryption(self.encryption);
        conf.authentication(self.auth_method);
        conf.trust_cert(); // TODO: Remove blindly trusting db

        if let Some(instance_name) = self.instance_name {
            conf.instance_name(instance_name);
        }
        if let Some(application_name) = self.application_name {
            conf.application_name(application_name);
        }

        conf
    }
}

use crate::error::MsSqlError;
use crate::pool::PoolConnection;

use crate::config::traits::ToPool;
use async_trait::async_trait;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type TokioClient = Client<Compat<TcpStream>>;

#[async_trait]
impl ToPool for MssqlConfig {
    type Error = MsSqlError;

    async fn to_pool(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
