use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub enum MachineType {
    #[serde(rename = "washer")]
    Washer,
    #[serde(rename = "dryer")]
    Dryer,
}

#[derive(Deserialize, Debug)]
pub enum ModeType {
    #[serde(rename = "pressStart")]
    PressStart,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub opaque_id: Uuid,
    pub location_id: Uuid,
    pub room_id: String,
    pub nfc_id: Uuid,
    pub qr_code_id: String,
    pub license_plate: String,
    pub sticker_number: i16,
    #[serde(rename = "type")]
    pub r#type: MachineType,
    pub door_closed: bool,
    pub available: bool,
    pub not_available_reason: Option<String>,
    pub mode: ModeType,
    pub time_remaining: Option<i16>,

    #[serde(flatten)]
    pub settings: MachineSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineSettings {
    pub soil: Option<String>,
    pub cycle: Option<String>,
    pub washer_temp: Option<String>,
    pub dryer_temp: Option<String>,
}

pub(crate) type MachineList = Vec<Machine>;

// TODO: Description might be null?
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiLocation {
    pub location_id: Uuid,
    pub description: String,
    pub label: String,
    pub rooms: Vec<ApiRoom>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiRoom {
    pub location_id: Uuid,
    pub room_id: String,
    pub description: String,
    pub label: String,
}

/// Struct for inserting into the database
#[derive(Deserialize, Debug, Eq, Hash, PartialEq)]
pub struct DbLocation {
    pub location_id: String,
    pub description: String,
    pub label: String,
}

/// Struct for inserting into the database
#[derive(Deserialize, Debug, Eq, Hash, PartialEq)]
pub struct DbRoom {
    pub room_id: String,
    pub description: String,
    pub label: String,
}
