use crate::types::RoomMachinesEndpoint;
use color_eyre::Result;
use reqwest::header::HeaderMap;
use reqwest::{Client, header};
use std::sync::LazyLock;
use tokio::task::{JoinSet, id};
use tokio::time::{Duration, sleep};
use tracing::{error, info, instrument, trace};

use crate::utils::cancel::*;

#[tracing::instrument(skip_all)]
pub(crate) async fn http_spawner(
    endpoints: Vec<RoomMachinesEndpoint>,
) -> Result<(JoinSet<()>, Vec<CancelSender>)> {
    let mut channels = vec![];
    let mut handles = JoinSet::new();

    info!("Spawning {N} http tasks.", N = endpoints.len());

    let client = Client::builder()
        .gzip(true)
        .default_headers(default_headers())
        .build()?;

    for endpoint in endpoints {
        let (tx, rx) = cancel_channel();

        // Client clones are cheap, uses arc under the hood and uses a pool.
        handles.spawn(http_task(endpoint, client.clone(), rx));
        channels.push(tx);
    }

    Ok((handles, channels))
}

/// Long-lived task that handles api scrapping
#[instrument(skip_all, fields(task_id=%id(), location_id = endpoint.location_id(), room_id = endpoint.room_id() ))]
async fn http_task(
    endpoint: RoomMachinesEndpoint,
    client: Client,
    mut cancel: CancelReceiver,
) -> () {
    info!("Starting http task");

    loop {
        info!("Running http task");
        let dur = get_minute_dur_offset();
        info!("Sleeping for {:?}", dur);

        // Cancel and delay logic
        tokio::select! {
            _ = sleep(dur) => {}
            _ = &mut cancel => break
        }

        let req = client.get(endpoint.build_url()).send();
        match req.await {
            Ok(res) => {
                trace!("{:?}", res);
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    }
    info!("http task canceled");
    ()
}

const MIN_OFFSET: u64 = 0;
const MAX_OFFSET: u64 = 9;
fn get_minute_dur_offset() -> Duration {
    let offset = rand::random_range(MIN_OFFSET..=MAX_OFFSET);
    Duration::from_secs(60 + offset)
}

fn default_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
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
    std::env::var("USER_AGENT").unwrap_or_else(|_|"Mozilla/5.0 (Linux; Android 11; Pixel 3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Mobile Safari/537.36".to_string())
});
