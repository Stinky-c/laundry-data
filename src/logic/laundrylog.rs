/*

use hashset
Machine table
present machine cache: when new machine, check in program cache. if not in cache, check db.
    if all machine cache checks fail, insert into database

Pep table
Use similar cache

 */
use crate::models::api::Machine;
use crate::models::db::{LAUNDRYLOG_INSERT, MACHINE_CHECK, MACHINE_INSERT, PEP_CHECK, PEP_INSERT};
use crate::pep::PhysicalEndpointId;
use crate::utils::cache::CacheSet;
use crate::utils::prelude::*;
use tokio_postgres::Client;

pub(crate) async fn new_log_entry(conn: &Client, cache_set: CacheSet, machine: Machine) -> () {
    if let None = cache_set.machine.get(&machine.opaque_id).await {
        debug!("Machine cache miss.");
        let query = conn
            .query_opt(
                &cache_set.with_statement(&conn, MACHINE_CHECK).await,
                &[&machine.opaque_id],
            )
            .await
            .unwrap();

        match query {
            None => {
                debug!(
                    "Machine database miss. Adding new machine {:?}",
                    machine.opaque_id
                );
                let _ = conn
                    .query(
                        &cache_set.with_statement(&conn, MACHINE_INSERT).await,
                        &[
                            &machine.opaque_id,
                            &machine.qr_code_id,
                            &machine.nfc_id,
                            &"TODO",
                            &machine.r#type,
                            &machine.license_plate,
                        ],
                    )
                    .await
                    .unwrap();
            }
            Some(_) => {
                debug!("Database hit. Inserting into cache");
                cache_set.machine.insert(machine.opaque_id, ()).await;
            }
        }
    }

    let pep = PhysicalEndpointId::from_machine(&machine);
    let pep_str = pep.calculate_pep().unwrap();

    if let None = cache_set.pep.get(&pep_str).await {
        debug!("Pep cache miss.");
        let query = conn
            .query_opt(
                &cache_set.with_statement(&conn, PEP_CHECK).await,
                &[&pep_str],
            )
            .await
            .unwrap();

        match query {
            None => {
                info!("Pep database miss. adding new pep '{:?}'", pep_str);
                let _ = conn
                    .query(
                        &cache_set.with_statement(&conn, PEP_INSERT).await,
                        &[
                            &pep_str,
                            &pep.sticker_number(),
                            &pep.machine_id(),
                            &pep.room_id(),
                            &pep.location_id(),
                        ],
                    )
                    .await
                    .unwrap();
                // cache_set.pep.insert(pep_str.clone(), ()).await;
            }
            Some(_) => {
                debug!("Database hit. Inserting into cache");
                cache_set.pep.insert(pep_str.clone(), ()).await;
            }
        }
    }

    let _ = conn
        .query(
            &cache_set.with_statement(&conn, LAUNDRYLOG_INSERT).await,
            &[
                &pep_str,
                &machine.time_remaining, // FIXME: Why is null. this stupid
                &machine.not_available_reason,
                &machine.door_closed,
                &machine.mode,
                &serde_json::Value::Null,
            ],
        )
        .await
        .unwrap();

    ()
}
