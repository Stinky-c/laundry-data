-- Enums converted to TEXT with CHECK constraints (SQLite does not support enums)
-- MachineState enum values: pressStart, running, idle, unknown
-- MachineType enum values: washer, dryer
-- Boolean columns converted to INTEGER (0 = false, 1 = true) (SQLite does not support bool)

CREATE TABLE Locations (
    location_id TEXT NOT NULL,
    description TEXT,
    label TEXT NOT NULL,
    timezone TEXT NOT NULL,
    PRIMARY KEY (location_id)
);

CREATE TABLE Rooms (
    room_id TEXT NOT NULL,
    description TEXT,
    label TEXT NOT NULL,
    PRIMARY KEY (room_id)
);

CREATE TABLE Machines (
    machine_id TEXT NOT NULL,
    qr_code_id TEXT NOT NULL,
    nfc_id TEXT NOT NULL,
    controller_type TEXT NOT NULL,
    type TEXT NOT NULL,
    license_plate TEXT NOT NULL,
    PRIMARY KEY (machine_id),
    CHECK (type IN ('washer', 'dryer')),
    CHECK (length(license_plate) <= 7)
);

CREATE TABLE PhysicalEndpoint (
    pep_id TEXT NOT NULL UNIQUE,
    added_on TEXT NOT NULL DEFAULT CURRENT_DATE,
    room_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    machine_id TEXT NOT NULL,
    sticker_number INTEGER NOT NULL,
    PRIMARY KEY (room_id, location_id, machine_id, sticker_number),
    FOREIGN KEY (room_id) REFERENCES Rooms(room_id),
    FOREIGN KEY (location_id) REFERENCES Locations(location_id),
    FOREIGN KEY (machine_id) REFERENCES Machines(machine_id)
);

CREATE TABLE LaundryLog (
    pep_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    time_remaining INTEGER NOT NULL,
    not_available_reason TEXT,
    door_closed INTEGER NOT NULL,
    state TEXT NOT NULL,
    machine_settings TEXT,
    PRIMARY KEY (pep_id, timestamp),
    CHECK (state IN ('pressStart', 'running', 'idle', 'unknown')),
    FOREIGN KEY (pep_id) REFERENCES PhysicalEndpoint(pep_id)
);
