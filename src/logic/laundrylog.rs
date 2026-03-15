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
use crate::utils::prelude::*;
use sql_middleware::{MiddlewarePoolConnection, QueryAndParams, RowValues};
use std::collections::HashSet;
use std::sync::{Arc, LazyLock, Mutex};
use uuid::Uuid;

// Reduce database calls
// Hashset cannot be built in a static/const context (uses random), lazylock provides lazily eval
type LazyArcMutexHashSet<T> = LazyLock<Arc<Mutex<HashSet<T>>>>;
static MACHINE_CACHE: LazyArcMutexHashSet<Uuid> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));
static PEP_CACHE: LazyArcMutexHashSet<String> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));

pub(crate) async fn new_log_entry(conn: &mut MiddlewarePoolConnection, machine: Machine) -> () {
    let machine_present: bool = machine_exists(conn, &machine).await;
    if !machine_present {
        warn!("machine {:?} does not exist", machine.opaque_id);
        match insert_new_machine(conn, &machine).await {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to insert machine")
            }
        };
    }

    let pep = PhysicalEndpointId::from_machine(&machine);
    let pep_str = pep.calculate_pep().unwrap();

    let pep_present = pep_exists(conn, &pep_str).await;
    if !pep_present {
        warn!("pep {:?} does not exist", pep_str);
        match insert_new_pep(conn, pep).await {
            Ok(_) => {}
            Err(e) => {
                panic!("insert new_pep failed: {:?}", e);
            }
        }
    }
    let reason_or_null = if let Some(reason) = machine.not_available_reason {
        RowValues::Text(reason)
    } else {
        RowValues::Null
    };
    let query = insert_log_entry(
        conn,
        vec![
            RowValues::Text(pep_str),
            RowValues::Int(machine.time_remaining.unwrap_or(0) as i64),
            reason_or_null,
            RowValues::Int(machine.door_closed as i64),
            RowValues::Text(machine.mode.variant_string()),
            RowValues::Text("{}".to_string()), // TODO: Machine settings
        ],
    );
    let _res = conn
        .query(&query.query)
        .params(&query.params)
        .dml()
        .await
        .unwrap();
    ()
}

async fn insert_new_machine(conn: &mut MiddlewarePoolConnection, machine: &Machine) -> Result<()> {
    let query = insert_machine(
        conn,
        vec![
            RowValues::Text(machine.opaque_id.to_string()),
            RowValues::Text(machine.qr_code_id.to_string()),
            RowValues::Text(machine.nfc_id.to_string()),
            RowValues::Text("TODO".to_string()),
            RowValues::Text(machine.r#type.variant_string()),
            RowValues::Text(machine.license_plate.to_string()),
        ],
    );
    info!("Adding new machine {:?}", machine.opaque_id);
    let _res = conn.query(&query.query).params(&query.params).dml().await?;

    Ok(())
}

async fn insert_new_pep(
    conn: &mut MiddlewarePoolConnection,
    pep: PhysicalEndpointId,
) -> Result<()> {
    let pep_str = pep.calculate_pep()?;
    info!("adding new pep '{:?}'", pep_str);
    let query = insert_pep(
        conn,
        vec![
            RowValues::Text(pep_str),
            RowValues::Int(pep.sticker_number() as i64),
            RowValues::Text(pep.machine_id().to_string()),
            RowValues::Text(pep.room_id().to_string()),
            RowValues::Text(pep.location_id().to_string()),
        ],
    );
    let _res = conn.query(&query.query).params(&query.params).dml().await?;

    Ok(())
}

async fn pep_exists(conn: &mut MiddlewarePoolConnection, pep_id: &String) -> bool {
    match { (*PEP_CACHE).lock().unwrap().contains(pep_id) } {
        true => {
            trace!("PEP Cache hit {:?}", pep_id);
            true
        }
        false => {
            // missed cache.
            // check db
            match pep_exists_in_db(conn, &pep_id).await {
                true => {
                    // in database, not cache
                    trace!("pep id db hit {:?}", pep_id);
                    info!("Adding pep to local cache {:?}", pep_id);
                    {
                        let mut cache = PEP_CACHE.lock().unwrap();
                        cache.insert(pep_id.to_string());
                    }
                    true
                }
                false => {
                    trace!("pep cache miss for {}", pep_id);
                    false
                }
            }
        }
    }
}

/// Checks if PhysicalEndpoint is present in database
/// Use [pep_exists].
async fn pep_exists_in_db(conn: &mut MiddlewarePoolConnection, pep_id: &str) -> bool {
    let query = select_pep_exists(conn, vec![RowValues::Text(pep_id.to_string())]);
    let v = conn
        .query(&query.query)
        .params(&query.params)
        .select()
        .await
        .unwrap();
    // If not empty, then row in database
    !v.results.is_empty()
}

async fn machine_exists(conn: &mut MiddlewarePoolConnection, machine: &Machine) -> bool {
    match {
        (*MACHINE_CACHE)
            .lock()
            .unwrap()
            .contains(&machine.opaque_id)
    } {
        true => {
            trace!("machine cache hit {:?}", machine.opaque_id);
            true
        }
        false => {
            // missed cache.
            // check db
            match machine_exists_in_db(conn, &machine.opaque_id).await {
                true => {
                    // in database, not cache
                    trace!("machine db hit {:?}", machine.opaque_id);
                    info!("Adding machine to local cache {:?}", machine.opaque_id);
                    {
                        let mut cache = MACHINE_CACHE.lock().unwrap();
                        cache.insert(machine.opaque_id);
                    }
                    true
                }
                false => {
                    trace!("machine cache miss for {:?}", machine.opaque_id);
                    false
                }
            }
        }
    }
}

/// Checks if machine_id exists in database
/// Use [machine_exists].
async fn machine_exists_in_db(conn: &mut MiddlewarePoolConnection, machine_id: &Uuid) -> bool {
    let query = select_machine_exists(conn, vec![RowValues::Text(machine_id.to_string())]);
    let v = conn
        .query(&query.query)
        .params(&query.params)
        .select()
        .await
        .unwrap();
    // If not empty, then row in database
    !v.results.is_empty()
}

fn select_machine_exists(
    conn: &mut MiddlewarePoolConnection,
    params: Vec<RowValues>,
) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => "SELECT * FROM Machines WHERE machine_id = @P1",
    };

    QueryAndParams::new(query, params)
}

fn select_pep_exists(
    conn: &mut MiddlewarePoolConnection,
    params: Vec<RowValues>,
) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => {
            "SELECT * FROM PhysicalEndpoint WHERE pep_id = @P1"
        }
    };
    QueryAndParams::new(query, params)
}

/// Follows the same order as [PhysicalEndpointId] except the string value is first.
fn insert_pep(conn: &mut MiddlewarePoolConnection, params: Vec<RowValues>) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => {
            r#"
            INSERT INTO PhysicalEndpoint(pep_id,added_on,sticker_number,machine_id,room_id,location_id)
             VALUES (@P1, GETDATE(), @P2, @P3, @P4, @P5)"#
        }
    };
    QueryAndParams::new(query, params)
}

fn insert_machine(conn: &mut MiddlewarePoolConnection, params: Vec<RowValues>) -> QueryAndParams {
    let query = match conn {
        MiddlewarePoolConnection::Mssql { .. } => {
            r#"
            INSERT INTO Machines(machine_id, qr_code_id, nfc_id, controller_type, [type], license_plate)
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6)"#
        }
    };
    QueryAndParams::new(query, params)
}

fn insert_log_entry(conn: &mut MiddlewarePoolConnection, params: Vec<RowValues>) -> QueryAndParams {
    let query = match conn {
        // TODO: Replace GETDATE() with better timestamping
        MiddlewarePoolConnection::Mssql { .. } => {
            r#"
            INSERT INTO LaundryLog(pep_id,timestamp,time_remaining,not_available_reason,door_closed,state,machine_settings)
            VALUES (@P1, GETDATE(), @P2, @P3, @P4, @P5, @P6)"#
        }
    };
    QueryAndParams::new(query, params)
}
