#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use color_eyre::{Report, Result};
    pub(crate) use tokio::task::id;
    pub(crate) use tokio_util::sync::CancellationToken;
    pub(crate) use tracing::{debug, error, info, instrument, trace, warn};
}

pub(crate) mod url {
    use crate::models::config::ApiConfig;

    pub fn machines(api_config: &ApiConfig, location_id: &str, room_id: &str) -> String {
        format!(
            "{proto}://{host}:{port}/api/v1/location/{location_id}/room/{room_id}/machines",
            proto = api_config.proto,
            host = api_config.host,
            port = api_config.port,
        )
    }
    pub fn location(api_config: &ApiConfig, location_id: &str) -> String {
        format!(
            "{proto}://{host}:{port}/api/v1/location/{location_id}",
            proto = api_config.proto,
            host = api_config.host,
            port = api_config.port,
        )
    }
}
