mod db;
mod logic;
mod types;
mod utils;

use crate::db::DbConfig;
use crate::types::{Http2DbTxRx, RoomMachinesEndpoint};
use color_eyre::eyre::{eyre, Result};
use sql_middleware::QueryAndParams;
use std::env::{var, VarError};
use tokio::signal::ctrl_c;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::logic::http::build_client;
use crate::utils::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

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

    let endpoints = vec![
        RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-001"),
        // RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-003"),
        // RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-004"),
        // RoomMachinesEndpoint::new("8ebe506e-0e8d-406c-bf4c-33e126ee38b4", "6321295-005"),
    ];
    // Cancel token for all sub-tasks
    let cancel_token = CancellationToken::new();
    let tracker = TaskTracker::new();
    let tracker_with_token = (tracker.clone(), cancel_token.clone());

    // Spawn tasks

    let (http_tx, http_rx) = tokio::sync::mpsc::channel(32);
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);

    // Http tasks
    let http_client = build_client()?;
    // Spawns scrappers inside
    logic::http::http_endpoints(tracker_with_token, http_client.clone(), endpoints, http_tx)?;
    tracker.spawn(logic::http::http_controller(
        db_rx,
        http_client,
        cancel_token.clone(),
    ));

    // db tasks

    tracker.spawn(logic::db::db_controller(
        pool,
        http_rx,
        db_tx,
        cancel_token.clone(),
    ));

    tracker.close();

    tokio::select! {
        _ = ctrl_c() => {
            info!("Got exit signal.");
            cancel_token.cancel();
            tracker.wait().await;

            info!("Task shutdown complete. Exiting...");
            Ok(())
        }
    }
}
