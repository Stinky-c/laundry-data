use crate::types::{
    Db2HttpMessage, Db2HttpReceiver, Http2DbMessage, Http2DbSender, RoomMachinesEndpoint,
    TrackerWithToken,
};
use crate::utils::prelude::*;
use reqwest::{header, Client};
use std::sync::LazyLock;
use tokio::time::{sleep, Duration};

// Long-lived controller task. Handles control messages from the database
#[instrument(skip_all, fields(task_id=%id()))]
pub(crate) async fn http_controller(
    mut control_rx: Db2HttpReceiver,
    client: Client,
    cancel_token: CancellationToken,
) -> () {
    info!("Initializing HTTP Control task");

    loop {
        let msg = tokio::select! {
            _ = cancel_token.cancelled() => {trace!("Got cancel");break},
            value = control_rx.recv() => {
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
            Db2HttpMessage::Noop => info!("DB NOOP"),
            Db2HttpMessage::MissingMachineIdent { .. } => unimplemented!(),
            Db2HttpMessage::MissingRoomIdent { .. } => unimplemented!(),
            Db2HttpMessage::MissingLocationIdent { .. } => unimplemented!(),
        };
    }
}

#[instrument(skip_all)]
pub(crate) fn http_endpoints(
    tracker: TrackerWithToken,
    client: Client,
    endpoints: Vec<RoomMachinesEndpoint>,
    control_tx: Http2DbSender,
) -> Result<()> {
    info!("Spawning {N} http tasks.", N = endpoints.len());

    for endpoint in endpoints {
        // Client clones are cheap, uses arc under the hood and uses a pool.
        tracker.0.spawn(scrape_task(
            endpoint,
            client.clone(),
            tracker.1.clone(),
            control_tx.clone(),
        ));
    }

    Ok(())
}

/// Long-lived task that handles api scrapping.
/// Does not handle extra api requests
#[instrument(skip_all, fields(task_id=%id(), location_id = endpoint.location_id(), room_id = endpoint.room_id()
))]
async fn scrape_task(
    endpoint: RoomMachinesEndpoint,
    client: Client,
    cancel_token: CancellationToken,
    control_tx: Http2DbSender,
) -> () {
    info!("Initializing http task");

    loop {
        info!("Running http task");
        let dur = get_minute_dur_offset();
        info!("Sleeping for {:?}", dur);

        // Cancel and delay logic
        tokio::select! {
            _ = cancel_token.cancelled() => {trace!("Got cancel");break},
            _ = sleep(dur) => {}
        }

        let req = client.get(endpoint.build_url()).send();
        match req.await {
            Ok(res) => {
                let _ = control_tx.send(Http2DbMessage::ApiResponse(res)).await;
            }
            Err(err) => {
                error!("{:?}", err); // TODO: better api error handling
            }
        }
    }
}

#[instrument(skip_all)]
pub(crate) fn build_client() -> Result<Client> {
    Ok(Client::builder()
        .gzip(true)
        .default_headers(default_headers())
        .build()?)
}

const MIN_OFFSET: i64 = -9;
const MAX_OFFSET: i64 = 9;
fn get_minute_dur_offset() -> Duration {
    let offset = rand::random_range(MIN_OFFSET..=MAX_OFFSET);
    let dur = (60 + offset) as u64; // How does this make it safe?
    Duration::from_secs(dur)
}

fn default_headers() -> header::HeaderMap {
    let mut map = header::HeaderMap::new();
    map.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    map.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static((*USER_AGENT).as_str()),
    );

    map
}

static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    std::env::var("USER_AGENT").unwrap_or_else(|_| "Mozilla/5.0 (Linux; Android 11; Pixel 3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Mobile Safari/537.36".to_string())
});
