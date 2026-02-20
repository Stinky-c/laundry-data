use reqwest::Response;
use std::sync::LazyLock;

static API_PROTO: LazyLock<String> =
    LazyLock::new(|| std::env::var("API_PROTO").unwrap_or_else(|_| "https".to_string()));
static API_HOST: LazyLock<String> =
    LazyLock::new(|| std::env::var("API_HOST").unwrap_or_else(|_| "mycscgo.com".to_string()));

/// Represents the api endpoint for all machines in a location + room
pub(crate) struct RoomMachinesEndpoint(String, String);

impl RoomMachinesEndpoint {
    pub(crate) fn new(location_id: impl Into<String>, room_id: impl Into<String>) -> Self {
        Self(location_id.into(), room_id.into())
    }
    pub(crate) fn build_url(&self) -> String {
        format!(
            "{proto}://{host}/api/v1/location/{location_id}/room/{room_id}/machines",
            proto = *API_PROTO,
            host = *API_HOST,
            location_id = self.0,
            room_id = self.1
        )
    }

    pub(crate) fn location_id(&self) -> &str {
        &self.0
    }
    pub(crate) fn room_id(&self) -> &str {
        &self.1
    }
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
