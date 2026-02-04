# Schema


![Database schema](assets/schema.svg)

## Rationale

### `ApiLog`

This table keeps track of API Responses. The initial purpose of this table 
is for notification tracking. The program will keep track of failed responses and log them in here.
Includes a http response, an inferred error, and an error message. Since this follows the 
[Log](#laundrylog) table, it contains a timestamp from the time when the response returns.


### `LaundryLog`

The primary table. Called `LaundryLog` because it contains a parsed and formatted version of API 
Responses. 


### `Machines`

Every [room](#rooms) has a varying amount of machines. This keeps track of all machines present in
[`LaundryLog`](#laundrylog). This is maintained in a separate table to ensure space for historic 
tracking. 

### `Rooms`

A room belongs to a [location](#locations). Maintains information like a labels, descriptions and machine type 
counts. Use a trigger to update [locations](#locations) sum of machines

### `Locations`

A location has many [Rooms](#rooms). For helper functions and locale, each room also stores the
timezone (`timezone`) it is in. Create constraint + function/procedure to check if valid timezone. 
Throw if not valid timezone. See `pg_timezone_names` Has a label, description, and machine sum.