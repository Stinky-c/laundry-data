pub static LOCATION_QUERY: &str = "SELECT location_id::TEXT FROM Locations";
pub static ROOMS_QUERY: &str = "SELECT room_id::TEXT FROM Rooms";
pub static LOCATION_INSERT: &str =
    "INSERT INTO Locations(location_id, description, label, timezone) VALUES ($1, $2, $3, $4)";

pub static MACHINE_CHECK: &str = "SELECT true FROM Machines WHERE machine_id = $1;";
pub static PEP_CHECK: &str = "SELECT true FROM PhysicalEndpoint WHERE pep_id = $1;";

pub static ROOM_INSERT: &str = "INSERT INTO rooms(room_id, description, label) VALUES ($1,$2, $3)";

pub static LAUNDRYLOG_INSERT: &str = "INSERT INTO LaundryLog(pep_id,timestamp,time_remaining,not_available_reason,door_closed,state,machine_settings)
            VALUES ($1, now(), $2, $3, $4, $5, $6)";

pub static MACHINE_INSERT: &str =
    "INSERT INTO Machines(machine_id, qr_code_id, nfc_id, controller_type, type, license_plate)
            VALUES ($1, $2, $3, $4, $5, $6)";

pub static PEP_INSERT: &str =
    "INSERT INTO PhysicalEndpoint(pep_id,added_on,sticker_number,machine_id,room_id,location_id)
             VALUES ($1, now(), $2, $3, $4, $5)";
