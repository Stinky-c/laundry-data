mod db;
mod http;
mod types;
mod utils;

use crate::db::DbConfig;
use crate::types::RoomMachinesEndpoint;
use color_eyre::eyre::{Result, eyre};
use std::env;
use std::env::VarError;
use tokio::signal::ctrl_c;
use tracing::{debug, info, instrument};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let worker_threads: usize = match env::var("TOKIO_WORKERS") {
        Ok(value) => value
            .parse()
            .map_err(|_| eyre!("failed to parse TOKIO_WORKERS")),
        Err(VarError::NotPresent) => Ok(4),
        Err(err) => return Err(err.into()),
    }?;

    let config = DbConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()?
        .block_on(async_main(config))
}

#[instrument(skip_all)]
async fn async_main(config: DbConfig) -> Result<()> {
    let _pool = db::new_pool(config.clone()).await?;

    let report = db::embedded::run_async(config).await?;
    info!("Migrations complete: {:?}", report);

    // let mut conn = pool.get_connection().await?;
    // let query = QueryAndParams::new_without_params("SELECT * FROM mssql");
    // let res = conn.query(&query.query).select().await?;
    // println!("{:?}", res);

    let endpoints = vec![
        RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-001"),
        RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-003"),
        RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-004"),
        RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-005"),
    ];

    let (http_join, http_cancel) = http::http_spawner(endpoints).await?;

    tokio::select! {
        _ = ctrl_c() => {
            info!("Got exit signal.");
            utils::cancel::vec_cancel(http_cancel);

            debug!("Sent cancel requests");
            http_join.join_all().await;

            info!("Task shutdown complete. Exiting...");
            Ok(())
        }
    }
}
