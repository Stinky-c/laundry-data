use crate::logic::laundrylog;
use crate::models;
use crate::models::api::{ApiLocation, DbLocation, DbRoom};
use crate::models::config::ApiConfig;
use crate::types::{Db2HttpMessage, Db2HttpSender, Http2DbMessage, Http2DbReceiver};
use crate::utils::cache::CacheSet;
use crate::utils::db::row_to_hashset;
use crate::utils::prelude::*;
use moka::future::Cache;
use reqwest::Response;
use std::collections::HashSet;
use tokio::sync::oneshot;
use tokio_postgres::Client;
use uuid::Uuid;

/// Controller for DB related tasks
#[instrument(skip_all, fields(task_id=%id()))]
pub(crate) async fn db_controller(
    api_config: ApiConfig,
    conn: Client,
    mut http_control_rx: Http2DbReceiver,
    db_control_tx: Db2HttpSender,
    cancel_token: CancellationToken,
) -> () {
    info!("Initializing DB Control task");

    db_precheck(api_config, &conn, db_control_tx.clone())
        .await
        .unwrap();

    // TODO: eviction listening? Prefill cache?
    let cache_set = CacheSet {
        rooms: Cache::<String, ()>::new(128),
        machine: Cache::<Uuid, ()>::new(128),
        pep: Cache::<String, ()>::new(128),
    };

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
            Http2DbMessage::ApiResponse(res) => {
                db_insert(&conn, cache_set.clone(), res).await;
            }
            Http2DbMessage::ApiError(_err) => unimplemented!(),
        };
    }

    // cleanup
}

#[instrument(skip_all)]
async fn db_insert(conn: &Client, cache_Set: CacheSet, response: Response) -> () {
    let body = response.json::<Vec<models::api::Machine>>().await.unwrap(); //FIXME: Actual parsing handler
    info!("Got new batch");
    for machine in body {
        laundrylog::new_log_entry(conn, cache_Set.clone(), machine).await;
    }
    info!("Batch complete")
}

#[instrument(skip_all)]
async fn db_precheck(endpoints: ApiConfig, conn: &Client, control_tx: Db2HttpSender) -> Result<()> {
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

    // locations and rooms found in database. Explict type cast to text so hashset can parse it
    let location_query = conn
        .query("SELECT location_id::TEXT FROM Locations", &[])
        .await?;
    let db_locations_set = row_to_hashset(location_query);
    let rooms_query = conn.query("SELECT room_id::TEXT FROM Rooms", &[]).await?;
    let db_rooms_set = row_to_hashset(rooms_query);

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
                location_id: recv.location_id,
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
        let insert = conn.query("INSERT INTO Locations(location_id, description, label, timezone) VALUES ($1, $2, $3, $4)", &[
            &loc.location_id,
            &"",
            &loc.label,
            &endpoints.tz
        ]).await;
        if let Err(e) = insert {
            error!("failed to insert location: {:?}", e)
        }
    }

    for room in found_rooms {
        let query = conn
            .query(
                "INSERT INTO rooms(room_id, description, label) VALUES ($1,$2, $3)",
                &[&room.room_id, &room.description, &room.label],
            )
            .await;
        if let Err(e) = query {
            error!("failed to insert room: {:?}", e)
        }
    }

    Ok(())
}
