use crate::models::api::Machine;
use crate::utils::prelude::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use uuid::Uuid;
use xxhash_rust::xxh3::xxh3_128;

pub(crate) struct PhysicalEndpointId {
    // Order follows, xxhash3 128 bit, base64 encoded
    // sticker[le], machine[uuid], room[string], location[uuid]
    sticker_number: u32,
    machine_id: Uuid,
    room_id: String,
    location_id: Uuid,
}

impl PhysicalEndpointId {
    pub fn new(sticker_number: u32, machine_id: Uuid, room_id: String, location_id: Uuid) -> Self {
        Self {
            sticker_number,
            machine_id,
            room_id,
            location_id,
        }
    }
    pub fn from_machine(machine: &Machine) -> Self {
        Self {
            sticker_number: machine.sticker_number as u32,
            machine_id: machine.opaque_id,
            room_id: machine.room_id.clone(),
            location_id: machine.location_id,
        }
    }
    pub fn calculate_pep(&self) -> Result<String> {
        let sticker_slice = self.sticker_number.to_le_bytes();
        let machine_slice = self.machine_id.as_bytes();
        let room_slice = self.room_id.as_bytes();
        let location_slice = self.location_id.as_bytes();

        let mut slice = vec![];
        slice.extend_from_slice(&sticker_slice);
        slice.extend_from_slice(machine_slice);
        slice.extend_from_slice(room_slice);
        slice.extend_from_slice(location_slice);

        let hash = xxh3_128(slice.as_slice());

        Ok(URL_SAFE_NO_PAD.encode(hash.to_le_bytes()))
    }
    pub(crate) fn sticker_number(&self) -> u32 {
        self.sticker_number
    }
    pub(crate) fn machine_id(&self) -> &Uuid {
        &self.machine_id
    }
    pub(crate) fn room_id(&self) -> &str {
        &self.room_id
    }
    pub(crate) fn location_id(&self) -> &Uuid {
        &self.location_id
    }
}
