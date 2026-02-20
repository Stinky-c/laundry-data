use crate::types::{Db2HttpSender, Http2DbMessage, Http2DbReceiver};
use crate::utils::prelude::*;
use sql_middleware::{ConfigAndPool, MiddlewarePoolConnection};

/// Controller for DB related tasks
#[instrument(skip_all,fields(task_id=%id()))]
pub(crate) async fn db_controller(
    pool: ConfigAndPool,
    mut http_control_rx: Http2DbReceiver,
    db_control_tx: Db2HttpSender,
    cancel_token: CancellationToken,
) -> () {
    info!("Initializing DB Control task");

    loop {
        let msg = tokio::select! {
            _ = cancel_token.cancelled() => {trace!("Got cancel");break},
            value = http_control_rx.recv() => {
                match value {
                    Some(v) => v,
                    None => {
                        error!("Channel closed unexpectedly");
                        break;
                    },
                }
            },
        };

        match msg {
            Http2DbMessage::ApiResponse(res) => {
                info!("Got response");
                trace!("{:?}", res);
            }

        }
    }

    // cleanup
}

// Attempt to insert into db, if a part doesn't exist yield and message parent.
// Wait for resume to re-attempt insert
#[tracing::instrument(skip_all)]
async fn db_insert(conn: MiddlewarePoolConnection, control_tx: Db2HttpSender) -> () {}

#[tracing::instrument(skip_all)]
async fn db_task(conn: MiddlewarePoolConnection, control_tx: Db2HttpSender) -> () {}
