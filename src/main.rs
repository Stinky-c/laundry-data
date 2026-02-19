mod db;
mod logic;
mod types;
mod utils;

use crate::db::DbConfig;
use crate::types::RoomMachinesEndpoint;
use color_eyre::eyre::{Result, eyre};
use std::env::{VarError, var};
use tokio::signal::ctrl_c;
use tokio_util::sync::CancellationToken;
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

    let worker_threads: usize = match var("TOKIO_WORKERS") {
        Ok(value) => value
            .parse()
            .map_err(|_| eyre!("failed to parse TOKIO_WORKERS")),
        Err(VarError::NotPresent) => Ok(6),
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
    info!("Beginning startup");
    let pool = db::new_pool(config.clone()).await?;
    // TODO: Check for database connectivity

    info!("Applying migrations");
    let report = db::embedded::run_async(config).await?;
    info!(
        "Migrations complete: Applied {} migrations",
        report.applied_migrations().len()
    );

    return Ok (());

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
    // Cancel token for all sub-tasks
    let cancel_token = CancellationToken::new();

    let (http_join, (control_tx, control_rx)) =
        logic::http_spawner(endpoints, cancel_token.clone()).await?;

    let (db_join,) =
        logic::db_spawner(pool, (control_tx.clone(), control_rx), cancel_token.clone()).await?;

    tokio::select! {
        _ = ctrl_c() => {
            info!("Got exit signal.");
            cancel_token.cancel();

            debug!("Sent cancel requests");
            // Maybe need to swap to a mutex
            tokio::join!(
                http_join.join_all(),
                db_join.join_all()
            );

            info!("Task shutdown complete. Exiting...");
            Ok(())
        }
    }
}
