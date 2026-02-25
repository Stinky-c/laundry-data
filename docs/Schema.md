# Schema


![Database schema](assets/schema.svg)


## Rationale

This is a brief description and reason behind all the columns and data types.

## `LaundryLog`

The log table. 

## `PhysicalEndpoint`

A tracking table that combines `location_id`, `room_id`,`machine_id`, and `sticker_number`.

## `Machines`

Insight into all machine seen. Depends on API Producers `machine_id` to be a unique primary key.

## `Rooms`

List of all tracked rooms. Depends on API Producer `room_id` to be unique.

## `Locations`

A location has many [Rooms](#rooms). For helper functions and locale, each room also stores the
timezone (`timezone`) it is in. Create constraint + function/procedure to check if valid timezone. 
Throw if not valid timezone. See `pg_timezone_names` Has a label, description, and machine sum.
