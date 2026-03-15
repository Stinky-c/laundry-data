use crate::logic::laundrylog;
use crate::models;
use crate::models::api::{ApiLocation, DbLocation, DbRoom};
use crate::models::config::ApiConfig;
use crate::types::{Db2HttpMessage, Db2HttpSender, Http2DbMessage, Http2DbReceiver};
use crate::utils::prelude::*;
use color_eyre::eyre::OptionExt;
use reqwest::Response;
use sql_middleware::{
    ConfigAndPool, MiddlewarePoolConnection, QueryAndParams, RowValues, TranslationMode,
};
use std::collections::HashSet;
use tokio::sync::oneshot;

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

        let mut insert_conn = pool.get_connection().await.unwrap();
        match msg {
            Http2DbMessage::ApiResponse(res) => {
                db_insert(&mut insert_conn, res).await;
            }
            Http2DbMessage::ApiError(_err) => unimplemented!(),
        };
    }

    // cleanup
}

#[instrument(skip_all)]
async fn db_insert(conn: &mut MiddlewarePoolConnection, response: Response) -> () {
    let body = response.json::<Vec<models::api::Machine>>().await.unwrap();
    info!("Got new batch");
    for machine in body {
        laundrylog::new_log_entry(conn, machine).await;
    }
    info!("Batch complete")
}

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
    let location_query = select_locations(&conn).query;
    let db_locations_set = get_query_as_hashset(&mut conn, &location_query).await?;
    let rooms_query = select_rooms(&conn).query;
    let db_rooms_set = get_query_as_hashset(&mut conn, &rooms_query).await?;

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
        let query = insert_location_query(
            &conn,
            vec![
                RowValues::Text(loc.location_id), // location_id
                RowValues::Null,                  // TODO: Description
                RowValues::Text(loc.label),       // label
            ],
        );
        let succ = conn
            .query(&query.query)
            .params(&query.params)
            .translation(TranslationMode::ForceOn)
            .dml()
            .await;
        if let Err(e) = succ {
            error!("failed to insert location: {:?}", e)
        }
    }

    for room in found_rooms {
        let query = insert_room_query(
            &conn,
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
        match value {
            RowValues::Text(val) => {
                set.insert(val.to_string());
            }
            v => panic!("got wrong type from query, {:?}", v),
        }
    }
    Ok(set)
}

fn select_rooms(conn: &MiddlewarePoolConnection) -> QueryAndParams {
    let query = match conn {
        _ => "SELECT room_id FROM rooms",
    };
    QueryAndParams::new_without_params(query)
}
fn select_locations(conn: &MiddlewarePoolConnection) -> QueryAndParams {
    let query = match conn {
        //FIXME: sql-middleware does not see unique identifier as a string
        MiddlewarePoolConnection::Mssql { .. } => {
            "SELECT lower(CAST(location_id AS VARCHAR(255))) AS location_id FROM Locations"
        }
        _ => "SELECT location_id FROM Locations",
    };
    QueryAndParams::new_without_params(query)
}

fn insert_room_query(conn: &MiddlewarePoolConnection, params: Vec<RowValues>) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => {
            "INSERT INTO rooms(room_id, description, label) VALUES (@P1,@P2, @P3)"
        }
    };
    QueryAndParams::new(query, params)
}

fn insert_location_query(
    conn: &MiddlewarePoolConnection,
    params: Vec<RowValues>,
) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => {
            "INSERT INTO locations(location_id, description, label, timezone) VALUES (@P1, @P2, @P3, 'UTC')"
        }
    };
    QueryAndParams::new(query, params)
}
