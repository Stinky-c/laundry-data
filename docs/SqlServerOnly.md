# Sql Server Extras

May be able to be used for other dialects but designed for SQL Server.

```tsql
SELECT *
from Locations;

SELECT *
from Rooms;

SELECT *
FROM Machines

SELECT *
FROM PhysicalEndpoint

-- List the name of room + location for every machine
SELECT sticker_number, Rooms.label, Locations.label
FROM Machines
         JOIN PhysicalEndpoint ON Machines.machine_id = PhysicalEndpoint.machine_id
         JOIN Rooms ON PhysicalEndpoint.room_id = Rooms.room_id
         JOIN Locations ON PhysicalEndpoint.location_id = Locations.location_id


--- Return all the latest log entries
CREATE FUNCTION func_LatestLogs()
    RETURNS TABLE
        AS RETURN(SELECT MAX([timestamp]) as [timestamp], pep_id
                  FROM LaundryLog
                  GROUP BY pep_id);

-- Join the latest entries back to the table
CREATE FUNCTION func_LatestLogsWithFields()
    RETURNS TABLE
        AS RETURN(SELECT LaundryLog.*
                  FROM dbo.LatestLogs() T
                           JOIN LaundryLog ON LaundryLog.timestamp = T.timestamp AND
                                              LaundryLog.pep_id = T.pep_id)


CREATE PROCEDURE sp_CurrentState(@pep_id NVARCHAR(255))
AS
BEGIN
    SELECT *
    FROM dbo.LatestLogsWithFields()
    WHERE pep_id = @pep_id
END;
GO

EXEC sp_currentState @pep_id = 'yZWM2OzN1dmasHzWWH2X2A';

-- Return all columns for a machine, room, and location given a pep
CREATE PROCEDURE sp_reversePep(@pep_id NVARCHAR(255))
AS
BEGIN
    SELECT *
    FROM PhysicalEndpoint
             JOIN Rooms ON Rooms.room_id = PhysicalEndpoint.room_id
             JOIN Machines ON Machines.machine_id = PhysicalEndpoint.machine_id
             JOIN Locations ON PhysicalEndpoint.location_id = Locations.location_id
    WHERE pep_id = @pep_id

END;
GO

EXEC sp_reversePep @pep_id = 'yZWM2OzN1dmasHzWWH2X2A';


-- Count number of machines in each room
SELECT COUNT(DISTINCT machines.machine_id) AS [count], PhysicalEndpoint.room_id
FROM Machines
         JOIN PhysicalEndpoint on Machines.machine_id = PhysicalEndpoint.machine_id
GROUP BY PhysicalEndpoint.room_id

-- Count Number of machines in every location
SELECT COUNT(DISTINCT machines.machine_id) AS [count], PhysicalEndpoint.location_id
FROM Machines
         JOIN PhysicalEndpoint on Machines.machine_id = PhysicalEndpoint.machine_id
GROUP BY PhysicalEndpoint.location_id

-- Function for counting every machine
CREATE FUNCTION func_CountEveryMachine() RETURNS INT
AS
BEGIN
    DECLARE @machineCount INT;
    SELECT @machineCount
               = COUNT(DISTINCT machine_id)
    FROM MACHINES
    RETURN @machineCount
END

-- Trigger version of machine counting
CREATE TABLE RoomMachineCount
(
    room_id nvarchar(255) unique,
    count   INT
);

-- Create a trigger that recalculates machine count for every room that had a machine insert
CREATE TRIGGER trigger_machineCount
    ON Machines
    AFTER INSERT , UPDATE
    AS
BEGIN
    -- Create a table of all affected rooms
    DECLARE @rooms TABLE
                   (
                       room_id NVARCHAR(255)
                   );
    INSERT INTO @rooms
    SELECT PhysicalEndpoint.room_id
    FROM inserted
             JOIN PhysicalEndpoint
                  ON inserted.machine_id = PhysicalEndpoint.machine_id

    -- Clean old data
    DELETE RoomMachineCount WHERE room_id in (@rooms)

    -- Update all affected rooms with new machine count
    INSERT INTO RoomMachineCount
    SELECT COUNT(DISTINCT machines.machine_id) AS [count], room_id
    FROM Machines
             JOIN PhysicalEndpoint ON PhysicalEndpoint.machine_id = Machines.machine_id
    WHERE PhysicalEndpoint.room_id IN (@rooms)
    GROUP BY PhysicalEndpoint.room_id

END
GO

DROP TABLE RoomMachineCount
DROP TRIGGER trigger_machineCount

-- Fast row counting
-- This table will grow fast so a less compute intensive method is needed
-- 45 machines * 1440 minutes (60 minutes * 24 hours * 1 day) =
-- = ~64800 per day
SELECT (dbo.func_CountEveryMachine() * (60 * 24)) AS 'RowsPerDay'

-- https://www.brentozar.com/archive/2014/02/count-number-rows-table-sql-server/
CREATE PROCEDURE sp_countRows(@object_id sysname) AS
BEGIN
    SELECT TBL.object_id, TBL.name, SUM(PART.rows) AS rows
    FROM sys.tables TBL
             INNER JOIN sys.partitions PART ON TBL.object_id = PART.object_id
             INNER JOIN sys.indexes IDX ON PART.object_id = IDX.object_id
        AND PART.index_id = IDX.index_id
    WHERE TBL.name = @object_id
      AND IDX.index_id < 2
    GROUP BY TBL.object_id, TBL.name;
END;

    EXEC sp_countRows @object_id = 'LaundryLog'
```