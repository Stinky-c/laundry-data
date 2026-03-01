# Wishes
Extra features I want. However, this project is for a grade.

## Tests

Likely to use a mock api serving JSON files. Design to be API compatible with CSC so testing is a 
simple endpoint swap. E2E style tests, data is put into the database and a test grabs to check

## Backfilling

This is a timescale feature. It helps fill in continuous data even if the source is not responding.
This solves the problem of when the CSC API stops responding to requests.

### Updating backfilled data
If backfilling is implemented, ensure that backfilled data is properly updated after service
resumes. For example, if a machine is started in the time frame where service is out then return to 
old data and update the row where a machine changes state.

## Load Claiming

A person can claim a load of laundry, which creates a laundry load for both washer and dryer
connecting it to a log table.

### Adds tables

- User
- LaundryLoad
- WasherLaundryLoads
- DryerLaundryLoad
- UserLogClaim

## Error Notifications

See [`ApiLog`](./Schema.md#apilog). A handler for the API Log to connect to a notification provider 
to send.