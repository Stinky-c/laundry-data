use crate::mssql_prelude::*;
use async_trait::async_trait;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type TokioClient = Client<Compat<TcpStream>>;

#[async_trait]
impl ToConnectionPool for MssqlConfig {
    type Error = MsSqlError;

    async fn init_connection(self) -> Result<PoolConnection, Self::Error> {
        let config = self.into_tiberius_config();
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let client = Client::connect(config, tcp.compat_write()).await?;
        Ok(PoolConnection::Mssql(client))
    }
}
