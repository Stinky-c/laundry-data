-- SQL Server 2022 Migration
-- Enums replaced with NVARCHAR columns + CHECK constraints (SQL Server does not support enums)
-- MachineState enum values: pressStart, running, idle, unknown
-- MachineType enum values: washer, dryer
-- JSON columns converted to NVARCHAR(MAX) (SQL Server 2022 does not have a JSON column type)
-- TIMESTAMPTZ columns converted to DATETIMEOFFSET (SQL Server should not use datetime/timestamp columns)
-- CURRENT_DATE default replaced with SYSDATETIMEOFFSET() (CURRENT_DATE is not supported in SQL Server; general rule 4)
-- text PK/FK columns use NVARCHAR(255); SQL Server requires a defined length for indexed/key columns
-- [timestamp] is quoted because TIMESTAMP is a reserved keyword in SQL Server

CREATE TABLE Locations (
    location_id UNIQUEIDENTIFIER NOT NULL,
    description NVARCHAR(MAX),
    label NVARCHAR(MAX) NOT NULL,
    timezone NVARCHAR(MAX) NOT NULL,
    PRIMARY KEY (location_id)
);

CREATE TABLE Rooms (
    room_id NVARCHAR(255) NOT NULL,
    description NVARCHAR(MAX),
    label NVARCHAR(MAX) NOT NULL,
    PRIMARY KEY (room_id)
);

CREATE TABLE Machines (
    machine_id UNIQUEIDENTIFIER NOT NULL,
    qr_code_id NVARCHAR(MAX) NOT NULL,
    nfc_id NVARCHAR(MAX) NOT NULL,
    controller_type NVARCHAR(MAX) NOT NULL,
    type NVARCHAR(MAX) NOT NULL,
    license_plate VARCHAR(7) NOT NULL,
    PRIMARY KEY (machine_id),
    CHECK (type IN ('washer', 'dryer')),
    CHECK (LEN(license_plate) <= 7)
);

CREATE TABLE PhysicalEndpoint (
    pep_id NVARCHAR(255) NOT NULL,
    added_on DATETIMEOFFSET NOT NULL DEFAULT SYSDATETIMEOFFSET(),
    room_id NVARCHAR(255) NOT NULL,
    location_id UNIQUEIDENTIFIER NOT NULL,
    machine_id UNIQUEIDENTIFIER NOT NULL,
    sticker_number INT NOT NULL,
    PRIMARY KEY (room_id, location_id, machine_id, sticker_number),
    CONSTRAINT UQ_PhysicalEndpoint_pep_id UNIQUE (pep_id),
    FOREIGN KEY (room_id) REFERENCES Rooms(room_id),
    FOREIGN KEY (location_id) REFERENCES Locations(location_id),
    FOREIGN KEY (machine_id) REFERENCES Machines(machine_id)
);

CREATE TABLE LaundryLog (
    pep_id NVARCHAR(255) NOT NULL,
    [timestamp] DATETIMEOFFSET NOT NULL,
    time_remaining SMALLINT NOT NULL,
    not_available_reason NVARCHAR(MAX),
    door_closed BIT NOT NULL,
    state NVARCHAR(MAX) NOT NULL,
    machine_settings NVARCHAR(MAX),
    PRIMARY KEY (pep_id, [timestamp]),
    CHECK (state IN ('pressStart', 'running', 'idle', 'unknown')),
    FOREIGN KEY (pep_id) REFERENCES PhysicalEndpoint(pep_id)
);
