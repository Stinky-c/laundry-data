# Laundry Data Rewrite

I will complete a long standing project for a final. Is it a bad idea? Likely.

## Tables

- Laundry 
  - The log table, takes API responses for every machine in every room. Pending a rename.
- Machines 
  - A list of every machine seen in the rooms. Includes historic machines; Opaque ID is CSC id.
- Locations 
  - A location is where someone has a contract with CSC.
- Rooms 
  - A location can have many rooms; A single room can have many machines.
- Views 
  - Several materialized views of per room machines. Likely will live in a different schema.
- API Log 
  - Stripped API Responses for notifications and CSC API health metrics. 

## Considerations

Optimized for Postgres, I need to translate to MS SQL for final grade. The program will have MS SQL compatibility built in.

A machine state change usually follows: ‘pressStart -> running -> idle -> pressStart’
A machine changes from ‘idle’ to 'pressStart’ when the door opens, it does not consider if laundry has been removed. Fixing this problem is out of the project's scope.


## Overview

A rewrite of my current laundry data that is better than storing API responses in a database. Actual tables created will increase in scope.

At a set interval, a program will collect laundry machine stats from the CSC APIfor a list of defined locations, and rooms. The program will be inserted into the Log table. Then constraints, checks, and other connections will ensure each location, room, and machine (possibly more) will appear in the correct table with complete information and relationships. In PG, timescale will ensure the Log table is optimized for the time series data and provides extra functions to query the time series data.

Some queries can include a list of machines available or busy at room or location. How long has a machine been running, waiting to be emptied or free (Includes limit, see considerations). Provide useful insights including connecting to weather statistics to see usage change during seasons, how often a machine is used, and if a machine is likely to need maintenance (Counting run cycles).



## Wishes

Features that are out of scope and can be finished in my free time

- Tests
  - Use mock api using json files.

- Backfilling 

- Extra Load claim table
  - A person can claim a load of laundry, which creates a laundry load for both washer and dryer connecting it to a log.
  - Adds tables; User, LaundryLoad, WasherLaundryLoads, DryerLaundryLoad UserLogClaim 

- True error notifications
  - A lot of uncertainty in data from API
  - Notified when something is beginning to fail 

