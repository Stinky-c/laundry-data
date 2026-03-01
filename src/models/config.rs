use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct AppConfig {
    pub(crate) db: crate::db::DbConfig,
    pub(crate) api: ApiConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ApiConfig {
    pub(crate) endpoints: Vec<crate::types::RoomMachinesEndpoint>,
    #[serde(default = "ApiConfig::default_api_proto")]
    pub(crate) proto: String,
    #[serde(default = "ApiConfig::default_api_host")]
    pub(crate) host: String,
    #[serde(default = "ApiConfig::default_api_port")]
    pub(crate) port: u16,
}

impl ApiConfig {
    fn default_api_proto() -> String {
        "https".to_string()
    }
    fn default_api_host() -> String {
        "mycscgo.com".to_string()
    }
    fn default_api_port() -> u16 {
        443
    }
}
