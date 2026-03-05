# sql-experiment

This crate is an experiment that supports a multitude of backends.

## Notes
Most core logic is enum dispatching to traits. The forward facing variants hold ownership over deadpool objects.

All pool logic is handled by [deadpool](https://crates.io/crates/deadpool).
While enum dispatching (stuff under the hood), uses [ambassador](https://docs.rs/ambassador/latest/ambassador/)


## Backends - Drivers
- [Sqlite](https://crates.io/crates/deadpool-sqlite/0.12.1)
- [Postgres](https://crates.io/crates/deadpool-postgres)
- [Sql Server](https://crates.io/crates/deadpool-tiberius)


## Reason
I wanted a universal interface for several databases with SQL Server support. 
[SQLx removed it](https://github.com/launchbadge/sqlx/discussions/1616) for a latter update and [sea-orm offers it as a commercial product](https://www.sea-ql.org/SeaORM-X/docs/introduction/orm/).
This library does not have any nice features, paid guaranties or faster speeds. It is designed to 
bring all the common drivers to a unified single interface.


## Wishes
These are features that I want to add.

- runtime independent
  - tiberius abstracts network transfer, i am using tokio for the mvp
- Migrations
  - Use the refinery crate for managing migrations
  - Multi database migrations
- Query builder?
  - Maybe. Some dialects have different syntax

