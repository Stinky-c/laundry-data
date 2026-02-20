use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum MachineType {
    #[serde(rename = "washer")]
    Washer,
    #[serde(rename = "dryer")]
    Dryer,
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineSettings {
    pub soil: Option<String>,
    pub cycle: Option<String>,
    pub washer_temp: Option<String>,
    pub dryer_temp: Option<String>,
}
