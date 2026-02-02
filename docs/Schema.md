# Schema


![Database schema](out/schema.svg)

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

TODO

### `Rooms`

TODO

### ``Locations``

A location has many [Rooms](#rooms). For helper functions, each room also stores the timezone (`timezone`) it 
is in, this is used for locale stuff for users. 