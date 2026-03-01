# sql-experiment

This crate is an experiment that supports a multitude of backends.

## Backends - Drivers
- Sqlite - [async-sqlite](https://crates.io/crates/async-sqlite)
- Postgres - [tokio-postgres](https://crates.io/crates/tokio-postgres)
- Sql Server - [tiberius](https://crates.io/crates/tiberius)


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

