# Schema


![Database schema](assets/schema.svg)


## Rationale

This is a brief description and reason behind all the columns and data types.

## `LaundryLog`

The log table. Uses composite primary key from `pep_id` and `timestamp`. Primary tracking table for
all machines.

| Column               | Data Type     | Purpose                                                          |
|----------------------|---------------|------------------------------------------------------------------|
| pep_id               | text          | The pep_id identifiy the machine in a location and position      |
| timestamp            | datetime      | The time for when this line was entered in                       |
| time_remaining       | int           | How many minutes left on the machine                             |
| not_available_reason | text nullable | The reason a machine is not available, will be null if available |
| door_closed          | bool          | If the door is opened or closed                                  |
| state                | MachineState  | What state the machine is in                                     |
| settings             | json          | The settings for a current cycle                                 |



## `PhysicalEndpoint`

A tracking table that combines `location_id`, `room_id`,`machine_id`, and `sticker_number`.
The `pep_id` or physical endpoint is derived from 4 other columns. The initial byte slice comes from 
converting sticker to LE, the machine_id UUID to bytes, converting the room id string 
(hyphen included) to bytes then the location id UUID to bytes adding them in that order. Using this 
slice, then hashing using xxhash3 128-bit and base64 encoding it with url safe no padding. This 
implementation may differ across language which is a bug. See `Cargo.toml` for package versions. 
Table includes a `added_on` datetime column to be able to look up a machine only given 3 components
of `pep_id`.

| column         | Data Type | Purpose                                      |
|----------------|-----------|----------------------------------------------|
| added_on       | datetime2 | Maintain when the pep was added to the table |
| pep_id         | text      | The base64 encoded pep_id                    |
| machine_id     | uuid      | A FK to the machines table                   |
| location_id    | uuid      | A FK to the locations table                  |
| room_id        | text      | A FK to the rooms table                      |
| sticker_number | int       | The machines postional sticker number        |


## `Machines`

Insight into all machine seen. Depends on API Producers `machine_id` to be a unique primary key.

| Column          | Data Type   | Purpose                                        |
|-----------------|-------------|------------------------------------------------|
| machine_id      | uuid        | The PK UUID returned from the API              |
| qr_code_id      | text        | The qr code short code to the machine          |
| nfc_id          | text        | The unique id given the nfc tag on the machine |
| controller_type | text        | The controller controling the machine          |
| type            | MachineType | Washer or dryer?                               |
| license_plate   | varchar(7)  | a license plate on the machine                 | 

## `Rooms`

List of all tracked rooms. Depends on API Producer `room_id` to be a unique primary key.

| Column      | Data Type     | Purpose                                   |
|-------------|---------------|-------------------------------------------|
| room_id     | text          | The primary key from the API              |
| description | text nullable | An optional field describing the location |
| label       | text          | The name of a location                    |


## `Locations`

A location has many [Rooms](#rooms). For helper functions and locale, each room also stores the
timezone (`timezone`) it is in. Create constraint + function/procedure to check if valid timezone. 
Throw if not valid timezone. See `pg_timezone_names` or `sys.time_zone_info`.
Has a label, description, and machine sum.

| Column      | Data Type     | Purpose                                |
|-------------|---------------|----------------------------------------|
| location_id | uuid          | The PK uuid from the API               |
| description | text nullable | An optional description                |
| label       | text          | The name of the location               |
| timezone    | text          | A convience field for converting times |
