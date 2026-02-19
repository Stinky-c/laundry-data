CREATE TYPE MachineState AS ENUM (
  'pressStart',
  'running',
  'idle',
  'unknown'
);

CREATE TYPE MachineType AS ENUM (
  'washer',
  'dryer'
);

CREATE TYPE ApiStatus AS ENUM (
  'up',
  'down',
  'failure',
  'unknown'
);

CREATE TABLE LaundryLog
(
    id                   uuid PRIMARY KEY      DEFAULT (uuidv7()),
    timestamp            timestamptz  NOT NULL,
    machine_id           int          NOT NULL,
    current_state        MachineState NOT NULL,
    time_remaining       smallint     NOT NULL,
    backfilled           bool         NOT NULL DEFAULT false,
    door_closed          bool         NOT NULL,
    available            bool         NOT NULL,
    not_available_reason text         NOT NULL,
    machine_settings     json         NOT NULL DEFAULT ({}),

    FOREIGN KEY (machine_id) REFERENCES Rooms (id)
);

CREATE TABLE Machines
(
    id             int PRIMARY KEY NOT NULL,
    room_id        int             NOT NULL,
    type           MachineType     NOT NULL,
    last_seen      timestamptz     NOT NULL,
    csc_id         uuid UNIQUE     NOT NULL,
    license_plate  varchar(7)      NOT NULL,
    sticker_number smallint        NOT NULL,
    nfc_id         uuid            NOT NULL,
    qr_code_id     text            NOT NULL,
    capability     json,
    stack          text,

    FOREIGN KEY (room_id) REFERENCES Rooms (id)
);

CREATE TABLE Rooms
(
    id           int PRIMARY KEY NOT NULL,
    csc_id       int             NOT NULL,
    description  text,
    washer_count smallint        NOT NULL,
    dryer_count  smallint        NOT NULL,
    location_id  int             NOT NULL,

    FOREIGN KEY (location_id) REFERENCES Locations (id)
);

CREATE TABLE Locations
(
    id         int PRIMARY KEY NOT NULL,
    csc_id     int             NOT NULL,
    address    text            NOT NULL,
    washer_sum smallint,
    dryer_sum  smallint,
    timezone   text            NOT NULL
);

CREATE TABLE ApiLog
(
    id        int PRIMARY KEY NOT NULL,
    timestamp timestamptz     NOT NULL,
    code      smallint        NOT NULL,
    status    ApiStatus       NOT NULL DEFAULT 'unknown',
    message   text            NOT NULL,
    json_hash text            NOT NULL
);


