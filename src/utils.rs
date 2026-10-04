#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use color_eyre::{Report, Result};
    pub(crate) use tokio::task::id;
    pub(crate) use tokio_util::sync::CancellationToken;
    pub(crate) use tracing::{debug, error, info, instrument, trace, warn};
}

pub(crate) mod url {
    use crate::models::config::ApiConfig;

    pub fn machines(api_config: &ApiConfig, location_id: &str, room_id: &str) -> String {
        format!(
            "{proto}://{host}:{port}/api/v1/location/{location_id}/room/{room_id}/machines",
            proto = api_config.proto,
            host = api_config.host,
            port = api_config.port,
        )
    }
    pub fn location(api_config: &ApiConfig, location_id: &str) -> String {
        format!(
            "{proto}://{host}:{port}/api/v1/location/{location_id}",
            proto = api_config.proto,
            host = api_config.host,
            port = api_config.port,
        )
    }
}

pub(crate) mod db {
    use std::collections::HashSet;
    use std::hash::Hash;
    use tokio_postgres::Row;
    use tokio_postgres::types::{FromSql, FromSqlOwned};

    /// Small helper that gets the first index of ever returned row and combines into a hashset
    /// Will panic if the type cannot be converted into [T]
    pub fn row_to_hashset<T: FromSqlOwned + Eq + Hash>(rows: Vec<Row>) -> HashSet<T> {
        let mut set = HashSet::new();

        rows.into_iter().for_each(|row| {
            let value: T = row.get(0);
            set.insert(value);
        });
        set
    }
}

pub(crate) mod cache {
    use moka::future::Cache;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct CacheSet {
        pub rooms: Cache<String, ()>,
        pub machine: Cache<Uuid, ()>,
        pub pep: Cache<String, ()>,
    }
}
