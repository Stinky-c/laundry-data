mod db;
mod logic;
mod models;
mod types;
mod utils;
mod pep;

use config::Config;
use tokio::signal::ctrl_c;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::models::config::AppConfig;
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

    let app_config = Config::builder()
        .add_source(
            config::Environment::default()
                .separator("_")
                .ignore_empty(true),
        )
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(app_config))
}

#[instrument(skip_all)]
async fn async_main(config: AppConfig) -> Result<()> {
    info!("Beginning startup");
    let pool = db::new_pool(config.db.clone()).await?;
    // TODO: Check for database connectivity

    info!("Applying migrations");
    let report = db::embedded::run_async(config.db.clone()).await?;
    info!(
        "Migrations complete: Applied {} migrations",
        report.applied_migrations().len()
    );

    // Cancel token for all sub-tasks
    let cancel_token = CancellationToken::new();
    let tracker = TaskTracker::new();
    let tracker_with_token = (tracker.clone(), cancel_token.clone());

    // Spawn tasks

    let (http_tx, http_rx) = tokio::sync::mpsc::channel(32);
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);

    // Http tasks
    let http_client = logic::http::build_client()?;
    // Spawns scrappers inside
    logic::http::http_endpoints(
        tracker_with_token,
        config.api.clone(),
        http_client.clone(),
        http_tx,
    )?;
    tracker.spawn(logic::http::http_controller(
        db_rx,
        config.api.clone(),
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
