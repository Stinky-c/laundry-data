/*

use hashset
Machine table
present machine cache: when new machine, check in program cache. if not in cache, check db.
    if all machine cache checks fail, insert into database

Pep table
Use similar cache

 */
use crate::models::api::Machine;
use crate::pep::PhysicalEndpointId;
use crate::utils::cache::CacheSet;
use crate::utils::prelude::*;
use moka::future::Cache;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use tokio_postgres::{Client, GenericClient, Row};
use uuid::Uuid;

pub(crate) async fn new_log_entry(conn: &Client, cache_set: CacheSet, machine: Machine) -> () {
    if let None = cache_set.machine.get(&machine.opaque_id).await {
        debug!("Machine cache miss.");
        let query = conn
            .query_opt(
                "SELECT true FROM Machines WHERE machine_id = $1;",
                &[&machine.opaque_id],
            )
            .await
            .unwrap();

        match query {
            None => {
                debug!("Database miss");
                insert_new_machine(conn, &machine).await.unwrap();
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
                "SELECT true FROM PhysicalEndpoint WHERE pep_id = $1;",
                &[&pep_str],
            )
            .await
            .unwrap();

        match query {
            None => {
                insert_new_pep(conn, pep).await.unwrap();
            }
            Some(_) => {
                debug!("Database hit. Inserting into cache");
                cache_set.pep.insert(pep_str.clone(), ()).await;
            }
        }
    }

    let query = conn.query("INSERT INTO LaundryLog(pep_id,timestamp,time_remaining,not_available_reason,door_closed,state,machine_settings)
            VALUES ($1, now(), $2, $3, $4, $5, $6)", &[
        &pep_str,
        &machine.time_remaining, // FIXME: Why is null. this stupid
        &machine.not_available_reason,
        &machine.door_closed,
        &machine.mode,
        &serde_json::Value::Null,
    ]).await.unwrap();

    ()
}

async fn insert_new_machine(conn: &Client, machine: &Machine) -> Result<()> {
    let _ = conn.query("INSERT INTO Machines(machine_id, qr_code_id, nfc_id, controller_type, type, license_plate)
            VALUES ($1, $2, $3, $4, $5, $6)", &[&machine.opaque_id, &machine.qr_code_id, &machine.nfc_id, &"TODO", &machine.r#type, &machine.license_plate]).await?;
    info!("Adding new machine {:?}", machine.opaque_id);

    Ok(())
}

async fn insert_new_pep(conn: &Client, pep: PhysicalEndpointId) -> Result<()> {
    let pep_str = pep.calculate_pep()?;
    let _ = conn.query("INSERT INTO PhysicalEndpoint(pep_id,added_on,sticker_number,machine_id,room_id,location_id)
             VALUES ($1, now(), $2, $3, $4, $5)", &[
        &pep_str, &pep.sticker_number(), &pep.machine_id(), &pep.room_id(), &pep.location_id()
    ]).await?;

    info!("adding new pep '{:?}'", pep_str);

    Ok(())
}
