const API_PROTO: &str = env!("API_PROTO");
const API_HOST: &str = env!("API_HOST");

/// Represents the api endpoint for all machines in a location + room
pub(crate) struct RoomMachinesEndpoint(String, String);

impl RoomMachinesEndpoint {
    pub(crate) fn new(location_id: impl Into<String>, room_id: impl Into<String>) -> Self {
        Self(location_id.into(), room_id.into())
    }
    pub(crate) fn build_url(&self) -> String {
        format!(
            "{API_PROTO}://{API_HOST}/api/v1/location/{location_id}/room/{room_id}/machines",
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
