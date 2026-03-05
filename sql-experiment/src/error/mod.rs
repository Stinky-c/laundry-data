#![allow(unused_imports)]
#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "mssql")]
pub use mssql::*;
#[cfg(feature = "postgres")]
pub use postgres::*;
#[cfg(feature = "sqlite")]
pub use sqlite::*;

#[derive(thiserror::Error, Debug)]
pub enum CommonError {

}
