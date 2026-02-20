use reqwest::Response;
use serde::Deserialize;
use std::sync::LazyLock;

/// Represents the api endpoint for all machines in a location + room
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RoomMachinesEndpoint {
    pub(crate) room_id: String,
    pub(crate) location_id: String,
}

/// One-liner for the common task tracker and token.
pub(crate) type TrackerWithToken = (
    tokio_util::task::TaskTracker,
    tokio_util::sync::CancellationToken,
);

use tokio::sync::mpsc;
pub(crate) type Http2DbSender = mpsc::Sender<Http2DbMessage>;
pub(crate) type Http2DbReceiver = mpsc::Receiver<Http2DbMessage>;
pub(crate) type Http2DbTxRx = (Http2DbSender, Http2DbReceiver);

// http -> db
pub(crate) enum Http2DbMessage {
    ApiResponse(Response),
}

pub(crate) type Db2HttpSender = mpsc::Sender<Db2HttpMessage>;
pub(crate) type Db2HttpReceiver = mpsc::Receiver<Db2HttpMessage>;
pub(crate) type Db2HttpTxRx = (Db2HttpSender, Db2HttpReceiver);

// db -> http
pub(crate) enum Db2HttpMessage {
    Noop,
    MissingMachineIdent {
        room_id: String,
        location_id: String,
        machine_id: String,
    },
    MissingRoomIdent {
        room_id: String,
        location_id: String,
    },
    MissingLocationIdent {
        location_id: String,
    },
}
