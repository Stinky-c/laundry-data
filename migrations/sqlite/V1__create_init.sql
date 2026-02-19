-- SQLITE Conversion standards. Apply these conditions when converting to sqlite.
-- 0. If any schema changes are made that do not follow the following conditions.
--      Document the changes in a comment at the end of the script.
-- 1. Foreign keys are defined on the table creation, and not in row. They are only created at the end
--      of the table using the FOREIGN KEY REFERENCES syntax.
-- 2. Primary keys, the columns named "id", are always going to have the integer data type with
--      primary key and autoincrement conditions. This does not include columns with "id" in the name.
-- 3. Any column that is of type UUID should be converted to TEXT.
-- 4. Any column that is of type JSON should be converted to TEXT. Do not include any checks for valid JSON
-- 5. Any column that is of type TIMESTAMPTZ and other time columns should be converted to TEXT.
-- 6. If any integer data type is missing, use INTEGER.
-- 7. If a column uses data type VARCHAR with a set length, add a constraint to ensure maintained length.
-- 8. Any enums present should be converted to a check on the column.
-- 9. If any dialect exclusive features are included remove them and add a comment documenting the removal.


PRAGMA foreign_keys = ON;

-- MachineState equivalent
-- Allowed: pressStart, running, idle, unknown

-- MachineType equivalent
-- Allowed: washer, dryer

-- ApiStatus equivalent
-- Allowed: up, down, failure, unknown

CREATE TABLE "LaundryLog"
(
    "id"                   INTEGER PRIMARY KEY AUTOINCREMENT,
    "timestamp"            TEXT    NOT NULL,             -- no timestamp column
    "machine_id"           INTEGER NOT NULL,

    "current_state"        TEXT    NOT NULL
        CHECK ("current_state" IN ('pressStart', 'running', 'idle', 'unknown')),

    "time_remaining"       INTEGER NOT NULL,

    "backfilled"           INTEGER NOT NULL DEFAULT 0
        CHECK ("backfilled" IN (0, 1)),

    "door_closed"          INTEGER NOT NULL
        CHECK ("door_closed" IN (0, 1)),

    "available"            INTEGER NOT NULL
        CHECK ("available" IN (0, 1)),

    "not_available_reason" TEXT    NOT NULL,

    "machine_settings"     TEXT    NOT NULL DEFAULT '{}'
        CHECK (json_valid("machine_settings")),

    FOREIGN KEY (machine_id) REFERENCES Machines (id)
);

CREATE TABLE "Machines"
(
    "id"             INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id"        INTEGER NOT NULL,
    "type"           TEXT    NOT NULL
        CHECK ("type" IN ('washer', 'dryer')),
    "last_seen"      TEXT    NOT NULL,        -- timestamptz -> TEXT
    "csc_id"         TEXT    NOT NULL UNIQUE, -- uuid -> TEXT
    "license_plate"  TEXT    NOT NULL
        CHECK (length("license_plate") = 7),
    "sticker_number" INTEGER NOT NULL,        -- smallint -> INTEGER
    "nfc_id"         TEXT    NOT NULL,        -- uuid -> TEXT
    "qr_code_id"     TEXT    NOT NULL,
    "capability"     TEXT,                    -- json -> TEXT
    "stack"          TEXT,

    FOREIGN KEY ("room_id") REFERENCES "Rooms" ("id")
);

CREATE TABLE "Rooms"
(
    "id"           INTEGER PRIMARY KEY AUTOINCREMENT,
    "csc_id"       INTEGER NOT NULL,
    "description"  TEXT,
    "washer_count" INTEGER NOT NULL, -- smallint -> INTEGER
    "dryer_count"  INTEGER NOT NULL, -- smallint -> INTEGER
    "location_id"  INTEGER NOT NULL,

    FOREIGN KEY ("location_id") REFERENCES "Locations" ("id")
);

CREATE TABLE "Locations"
(
    "id"         INTEGER PRIMARY KEY AUTOINCREMENT,
    "csc_id"     INTEGER NOT NULL,
    "address"    TEXT    NOT NULL,
    "washer_sum" INTEGER, -- smallint -> INTEGER
    "dryer_sum"  INTEGER, -- smallint -> INTEGER
    "timezone"   TEXT    NOT NULL
);

CREATE TABLE "ApiLog"
(
    "id"        INTEGER PRIMARY KEY AUTOINCREMENT,
    "timestamp" TEXT    NOT NULL,
    "code"      INTEGER NOT NULL, -- smallint -> INTEGER
    "status"    TEXT    NOT NULL DEFAULT 'unknown'
        CHECK ("status" IN ('up', 'down', 'failure', 'unknown')),
    "message"   TEXT    NOT NULL,
    "json_hash" TEXT    NOT NULL
);
