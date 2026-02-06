# Considerations

- Use stream API for real time updates
- Support MsSQl for final grade
- Single timestamp for entire insert into [`ApiLog`](./Schema.md#apilog) and [`LaundryLog`](./Schema.md#laundrylog)
- [`machines`](./Schema.md#machines) may have a collision. If a machine is replaced in a room, both the `room_id` and `sticker_number` will be the same.
  - Solve using `last_seen` timestamp and add constraint to block collisions.
- Create views for rooms and locations dynamically.
  - use [`format()`](https://www.postgresql.org/docs/current/functions-string.html#FUNCTIONS-STRING-FORMAT).
  - Possible trigger on insert to [`rooms`](./Schema.md#rooms) or [`locations`](./Schema.md#locations) based on custom label?

## Machine state transitions

A machine follows usually follows a cycle of state changes. 
The cycle looks like ``pressStart => running => idle => pressStart``.
An important note, when the machine completes and moves to the `idle` state, opening the door moves 
it to the `pressStart` state even if it was never emptied. 
This means someone can leave their laundry inside the machine after opening the door and it will be
seen as free. Fixing this problem is out of the project's scope.