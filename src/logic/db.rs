use crate::models::api::{ApiLocation, DbLocation, DbRoom};
use crate::models::config::ApiConfig;
use crate::types::{Db2HttpMessage, Db2HttpSender, Http2DbMessage, Http2DbReceiver};
use crate::utils::prelude::*;
use color_eyre::eyre::OptionExt;
use sql_middleware::{
    ConfigAndPool, MiddlewarePoolConnection, QueryAndParams, RowValues, execute_batch,
};
use std::collections::HashSet;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

/// Controller for DB related tasks
#[instrument(skip_all, fields(task_id=%id()))]
pub(crate) async fn db_controller(
    api_config: ApiConfig,
    pool: ConfigAndPool,
    mut http_control_rx: Http2DbReceiver,
    db_control_tx: Db2HttpSender,
    cancel_token: CancellationToken,
) -> () {
    info!("Initializing DB Control task");

    db_precheck(
        api_config,
        pool.get_connection().await.unwrap(),
        db_control_tx.clone(),
    )
    .await
    .unwrap();

    loop {
        let msg = tokio::select! {
            _ = cancel_token.cancelled() => {debug!("Got cancel");break},
            value = http_control_rx.recv() => {
                match value {
                    Some(v) => v,
                    None => {
                        error!("Channel closed unexpectedly");
                        break;
                    },
                }
            },
        };

        match msg {
            Http2DbMessage::ApiResponse(_res) => {}
            Http2DbMessage::ApiError(_err) => unimplemented!(),
        };
    }

    // cleanup
}

// Attempt to insert into db, if a part doesn't exist yield and message parent.
// Wait for resume to re-attempt insert
#[instrument(skip_all)]
async fn db_insert(conn: MiddlewarePoolConnection, control_tx: Db2HttpSender) -> () {}

#[instrument(skip_all)]
async fn db_task(conn: MiddlewarePoolConnection, control_tx: Db2HttpSender) -> () {}

#[instrument(skip_all)]
async fn db_precheck(
    endpoints: ApiConfig,
    mut conn: MiddlewarePoolConnection,
    control_tx: Db2HttpSender,
) -> Result<()> {
    // locations and rooms found in config
    let (config_locations_set, config_rooms_set): (HashSet<String>, HashSet<String>) = {
        let mut locs = HashSet::new();
        let mut rooms = HashSet::new();

        for endpoint in endpoints.endpoints {
            locs.insert(endpoint.location_id);
            rooms.insert(endpoint.room_id);
        }
        (locs, rooms)
    };

    // locations and rooms found in database
    let db_locations_set = get_query_as_hashset(&mut conn, MISSING_LOCATIONS_QUERY).await?;
    let db_rooms_set = get_query_as_hashset(&mut conn, MISSING_ROOMS_QUERY).await?;

    // locations and rooms not present in the database, but found in config
    let missing_locations: HashSet<_> = config_locations_set
        .difference(&db_locations_set)
        .cloned()
        .collect();
    let missing_rooms: HashSet<_> = config_rooms_set
        .difference(&db_rooms_set)
        .cloned()
        .collect();

    info!("MISSING LOCATIONS: {:?}", missing_locations);
    info!("MISSING ROOMS: {:?}", missing_rooms);

    let mut found_locations: HashSet<DbLocation> = HashSet::new();
    let mut found_rooms: HashSet<DbRoom> = HashSet::new();

    // iter over all location ids in the config.
    // the set of missing rooms can only be missing if the location and room is found in the config
    for location in config_locations_set {
        let (once_tx, mut once_rx) = oneshot::channel::<ApiLocation>();
        // Ask http for the missing location/room data
        let control_res = control_tx
            .send(Db2HttpMessage::MissingRoomLocationIdent {
                location_id: location.to_string(),
                return_channel: once_tx,
            })
            .await;


        if let Err(e) = control_res {
            error!("Db2Http send error {:?}", e);
            continue;
        }

        let recv = match once_rx.await {
            Ok(v) => v,
            Err(e) => {
                error!("Db2Http return channel error {:?}", e);
                continue;
            }
        };

        // If the location was missing from db, add to the set to insert into db
        if missing_locations.contains(&recv.location_id.to_string()) {
            found_locations.insert(DbLocation {
                location_id: recv.location_id.to_string(),
                description: recv.description, // TODO
                label: recv.label,
            });
        }

        let filtered_rooms = recv
            .rooms
            .into_iter()
            .filter(|v| missing_rooms.contains(&v.room_id))
            .map(|v| DbRoom {
                room_id: v.room_id,
                description: v.description,
                label: v.label,
            });

        found_rooms.extend(filtered_rooms);
    }

    info!("FOUND LOCATIONS: {:?}", found_locations);
    info!("FOUND ROOMS: {:?}", found_rooms);

    for loc in found_locations {
        let query = QueryAndParams::new(
            INSERT_LOCATION_QUERY,
            vec![
                RowValues::Text(loc.location_id), // location_id
                RowValues::Null,                  // TODO: Description
                RowValues::Text(loc.label),       // label
            ],
        );
        let succ = conn.query(&query.query).params(&query.params).dml().await;
        if let Err(e) = succ {
            error!("failed to insert location: {:?}", e)
        }
    }

    for room in found_rooms {
        let query = QueryAndParams::new(
            INSERT_ROOM_QUERY,
            vec![
                RowValues::Text(room.room_id),     // room_id
                RowValues::Text(room.description), // TODO: Description
                RowValues::Text(room.label),       // label
            ],
        );
        let succ = conn.query(&query.query).params(&query.params).dml().await;
        if let Err(e) = succ {
            error!("failed to insert location: {:?}", e)
        }
    }

    Ok(())
}

async fn get_query_as_hashset(
    conn: &mut MiddlewarePoolConnection,
    query: &str,
) -> Result<HashSet<String>> {
    let result = conn.query(query).select().await?;
    let mut set: HashSet<String> = HashSet::new();

    for row in result.results.iter() {
        let value = row
            .get_by_index(0)
            .ok_or_eyre("Failed to get row by index 0")?;
        set.insert(
            value
                .as_text()
                .ok_or_eyre("Failed to convert to text")?
                .to_string(),
        );
    }
    Ok(set)
}

const MISSING_ROOMS_QUERY: &str = "SELECT room_id FROM rooms";
const MISSING_LOCATIONS_QUERY: &str = "SELECT location_id FROM locations";

const INSERT_ROOM_QUERY: &str =
    "INSERT INTO rooms(room_id, description, label) VALUES ($1, $2, $3)";
const INSERT_LOCATION_QUERY: &str =
    "INSERT INTO locations(location_id, description, label, timezone) VALUES ($1, $2, $3, 'UTC')";
