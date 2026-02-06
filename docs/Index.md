# Index

A rewrite of my current laundry data that is better than storing API responses in a database.
Actual tables created will increase in scope.

At a set interval, a program will collect laundry machine stats from the CSC API for a list of
defined locations, and rooms. The program will insert into the Log table.
Then constraints, checks, and other connections will ensure each location, room, and machine
(possibly more) will appear in the correct table with complete information and relationships.
In PG, timescale will ensure the [`LaundryLog`](./Schema.md#laundrylog) table is optimized for
time series data and provide extra functions to query the data.

Some queries can include a list of machines available or busy at room or location. How long has a machine been running, waiting to be emptied or free (Includes limit, see considerations). Provide useful insights including connecting to weather statistics to see usage change during seasons, how often a machine is used, and if a machine is likely to need maintenance (Counting run cycles).


## Notes to self

- Migrate from dbml file to generating from database. 

