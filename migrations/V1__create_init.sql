-- Enums converted to TEXT with CHECK constraints (SQLite does not support enums)
-- MachineState enum values: pressStart, running, idle, unknown
-- MachineType enum values: washer, dryer
-- Boolean columns converted to INTEGER (0 = false, 1 = true) (SQLite does not support bool)


CREATE TYPE machine_state AS ENUM ('pressStart', 'running', 'idle', 'unknown');
CREATE TYPE machine_type AS ENUM ('washer', 'dryer');

CREATE TABLE Locations
(
    location_id UUID NOT NULL,
    description TEXT,
    label       TEXT NOT NULL,
    timezone    TEXT NOT NULL,
    PRIMARY KEY (location_id)
);

CREATE TABLE Rooms
(
    room_id     TEXT NOT NULL,
    description TEXT,
    label       TEXT NOT NULL,
    PRIMARY KEY (room_id)
);

CREATE TABLE Machines
(
    machine_id      UUID         NOT NULL,
    qr_code_id      TEXT         NOT NULL,
    nfc_id          UUID         NOT NULL,
    controller_type TEXT         NOT NULL,
    type            machine_type NOT NULL,
    license_plate   TEXT         NOT NULL,
    PRIMARY KEY (machine_id),
    CHECK (length(license_plate) <= 7)
);

CREATE TABLE PhysicalEndpoint
(
    pep_id         TEXT    NOT NULL UNIQUE,
    added_on       TEXT    NOT NULL DEFAULT CURRENT_DATE,
    room_id        TEXT    NOT NULL,
    location_id    UUID    NOT NULL,
    machine_id     UUID    NOT NULL,
    sticker_number INTEGER NOT NULL,
    PRIMARY KEY (room_id, location_id, machine_id, sticker_number),
    FOREIGN KEY (room_id) REFERENCES Rooms (room_id),
    FOREIGN KEY (location_id) REFERENCES Locations (location_id),
    FOREIGN KEY (machine_id) REFERENCES Machines (machine_id)
);

CREATE TABLE LaundryLog
(
    pep_id               TEXT          NOT NULL,
    timestamp            TIMESTAMPTZ   NOT NULL,
    time_remaining       SMALLINT      NOT NULL,
    not_available_reason TEXT,
    door_closed          BOOLEAN       NOT NULL,
    state                machine_state NOT NULL,
    machine_settings     JSON,
    PRIMARY KEY (pep_id, timestamp),
    FOREIGN KEY (pep_id) REFERENCES PhysicalEndpoint (pep_id)
);
