use crate::types::{
    ControlMessage, ControlMessageReceiver, ControlMessageSender, ControlMessageTxRx,
};
use crate::utils::prelude::*;
use sql_middleware::{ConfigAndPool, MiddlewarePoolConnection, SqlMiddlewareDbError};
use tokio::task::JoinSet;

#[tracing::instrument(skip_all)]
pub(crate) async fn db_spawner(
    pool: ConfigAndPool,
    control_channel: ControlMessageTxRx,
    cancel_token: CancellationToken,
) -> Result<(JoinSet<()>,)> {
    let mut set = JoinSet::new();

    info!("Spawning DB control task");

    set.spawn(db_controller(pool, control_channel, cancel_token.clone()));
    Ok((set,))
}

/// Controller for DB related tasks
#[tracing::instrument(skip_all)]
async fn db_controller(
    pool: ConfigAndPool,
    (control_tx, mut control_rx): ControlMessageTxRx,
    cancel_token: CancellationToken,
) -> () {
    // init
    info!("Initializing DB Control task");
    let mut set: JoinSet<()> = JoinSet::new();

    loop {
        let msg = tokio::select! {
            _ = cancel_token.cancelled() => break,
            msg = control_rx.recv() => {
                match msg {
                    None => {
                        info!("Control channel has been closed");
                        break;
                    },
                    Some(msg) => {msg}}}
        };
        match msg {
            ControlMessage::ApiResponse => {
                info!("Got response");
            }
            ControlMessage::MissingMachineIdent => unimplemented!(),
            ControlMessage::MissingRoomIdent => unimplemented!(),
            ControlMessage::MissingLocationIdent => unimplemented!(),

            _ => unimplemented!(),
        }
    }

    // cleanup
    ()
}

// Attempt to insert into db, if a part doesn't exist yield and message parent.
// Wait for resume to re-attempt insert
#[tracing::instrument(skip_all)]
async fn db_insert(conn: MiddlewarePoolConnection, control_tx: ControlMessageSender) -> () {}

#[tracing::instrument(skip_all)]
async fn db_task(conn: MiddlewarePoolConnection, control_tx: ControlMessageSender) -> () {}
