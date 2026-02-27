# Schema


![Database schema](assets/schema.svg)


## Rationale

This is a brief description and reason behind all the columns and data types.

## `LaundryLog`

The log table. Uses composite primary key from `pep_id` and `timestamp`. Primary tracking table for
all machines.

## `PhysicalEndpoint`

A tracking table that combines `location_id`, `room_id`,`machine_id`, and `sticker_number`.
The `pep_id` or physical endpoint is derived from 4 other columns. The initial byte slice comes from 
converting sticker to LE, the machine_id UUID to bytes, converting the room id string 
(hyphen included) to bytes then the location id UUID to bytes adding them in that order. Using this 
slice, then hashing using xxhash3 128-bit and base64 encoding it with url safe no padding. This 
implementation may differ across language which is a bug. See `Cargo.toml` for package versions. 
Table includes a `added_on` datetime column to be able to look up a machine only given 3 components
of `pep_id`.

## `Machines`

Insight into all machine seen. Depends on API Producers `machine_id` to be a unique primary key.

## `Rooms`

List of all tracked rooms. Depends on API Producer `room_id` to be a unique primary key.

## `Locations`

A location has many [Rooms](#rooms). For helper functions and locale, each room also stores the
timezone (`timezone`) it is in. Create constraint + function/procedure to check if valid timezone. 
Throw if not valid timezone. See `pg_timezone_names` or `sys.time_zone_info`.
Has a label, description, and machine sum.
