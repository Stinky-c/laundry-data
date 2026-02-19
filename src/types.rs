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


/// Multiple producer - single consumer. Safe to clone
pub(crate) type ControlMessageSender = tokio::sync::mpsc::Sender<ControlMessage>;

/// Multiple producer - single consumer. Unsafe to clone
pub(crate) type ControlMessageReceiver = tokio::sync::mpsc::Receiver<ControlMessage>;

/// Tuple of sender Tx, and Rx
pub(crate) type ControlMessageTxRx = (ControlMessageSender, ControlMessageReceiver);

#[non_exhaustive]
pub(crate) enum ControlMessage {
    ApiResponse,
    MissingMachineIdent,
    MissingRoomIdent,
    MissingLocationIdent
}
