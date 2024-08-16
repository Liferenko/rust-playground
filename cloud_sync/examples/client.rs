use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{timeout, Duration};
use warp::Filter; // Using warp for HTTP handling

#[derive(Clone)]
struct SyncService {
    // use it for state sharing
    waiting_parties: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

#[tokio::main]
async fn main() {
    let service = SyncService {
        waiting_parties: Arc::new(Mutex::new(HashMap::new())),
    };

    let sync_route = warp::path!("wait-for-second-party" / String)
        .and(warp::post())
        .and(with_service(service.clone()))
        .and_then(handle_request);

    warp::serve(sync_route).run(([127, 0, 0, 1], 4012)).await;
    println!("Server is up. Waiting for requests");
}

fn with_service(
    service: SyncService,
) -> impl Filter<Extract = (SyncService,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}

async fn handle_request(
    unique_id: String,
    service: SyncService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (transmitter, receiver) = oneshot::channel();

    let mut waiting_parties = service.waiting_parties.lock().await;

    if let Some(sender) = waiting_parties.remove(&unique_id) {
        // removed sender means the party is already waiting - so notify them
        sender.send(()).unwrap_or_default();
        Ok(warp::reply::json(
            &"2nd party connected. 1st and 2nd are synced now.",
        ))
    } else {
        // If noone is waiting - store the sender and wait
        waiting_parties.insert(unique_id.clone(), transmitter);
        drop(waiting_parties);

        // Wait for the 2nd party or timeout
        match timeout(Duration::from_secs(10), receiver).await {
            // TODO: send response to 1st party when time is up
            Ok(_) => Ok(warp::reply::json(&"Both parties synced successfully.")),
            Err(_) => {
                // Cleanup if timeout occurs
                service.cleanup(unique_id).await;
                Ok(warp::reply::json(&"Timeout waiting for the second party."))
            }
        }
    }
}

impl SyncService {
    async fn cleanup(&self, unique_id: String) {
        let mut waiting_parties = self.waiting_parties.lock().await;
        waiting_parties.remove(&unique_id);
    }
}

// #[tokio::test]
// async fn asdf() {
//     use std::time::{Duration, Instant};
//     // tokio::time::pause();
//     let start = Instant::now();
//     tokio::time::sleep(Duration::from_millis(1000)).await;
//     println!("{:?}ms", start.elapsed().as_millis());
// }

// TODO: REMOVE BEFORE FLIGHT!!!!!!
// #[cfg(test)]
// mod client_test;
