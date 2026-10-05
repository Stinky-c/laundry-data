mod db;
mod logic;
mod models;
mod pep;
mod types;
mod utils;

use crate::models::config::AppConfig;
use crate::utils::prelude::*;
use config::Config;
use tokio::signal::ctrl_c;
use tokio_postgres::NoTls;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{Instrument, trace_span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
#[instrument(ret, name = "laundry")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        // .with(console_layer)
        .with(fmt::layer().with_line_number(true))
        .with(EnvFilter::from_default_env())
        .init();

    let config: AppConfig = Config::builder()
        .add_source(
            config::Environment::default()
                .separator("_")
                .ignore_empty(true),
        )
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()?;

    info!("Beginning startup");
    debug!("Config: {:?}", &config);

    // Cancel token for all sub-tasks
    let cancel_token = CancellationToken::new();
    let tracker: TaskTracker = TaskTracker::new();
    let tracker_with_token = (tracker.clone(), cancel_token.clone());

    {
        // Cancel all other tasks when panicking
        let hook_tracker = tracker.clone();
        let hook_cancel_token = cancel_token.clone();
        let previous_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            hook_tracker.close();
            hook_cancel_token.cancel();

            previous_hook(panic_info);
        }))
    }

    let (mut client, connection) = tokio_postgres::config::Config::new()
        .host(config.db.host)
        .port(config.db.port)
        .dbname(config.db.db_name)
        .user(config.db.user_name)
        .password(config.db.password)
        .connect(NoTls)
        .await?;

    let db_cancel_token = cancel_token.clone();
    tracker.spawn(async move {
        tokio::select! {
            _ = db_cancel_token.cancelled() => {debug!("Canceling");}
            err = connection => {
                db_cancel_token.cancel();
                panic!("Database connection had an error. {err:#?}"); // Cant really do anything at this point
            }
        }
    });

    client.check_connection().await?;

    info!("Applying migrations");
    let report = db::migrations::runner()
        .run_async(&mut client)
        .instrument(trace_span!("migration"))
        .await?;
    info!(
        "Migrations complete: Applied {} migrations",
        report.applied_migrations().len()
    );

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
        config.api.clone(),
        client,
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

// TODO: find how to stop on a panic
// TODO: Log appending if wanted.
