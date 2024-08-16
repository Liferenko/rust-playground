// TODO: add entrypoint test: send req, send resp, show its content
//

use std::future::{self, Future};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
// use wasm_bindgen_test::console_log;
use web_sys::console;

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use log::{error, info};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

// pub struct Promise<T: Send, E: Send> {/* TODO: */}
// // https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[tokio::main]
#[wasm_bindgen(start)]
async fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Call you await
    // TODO: smells like poopy. Need to rework it
    let response = ws_ping("/snacks", "Spicy chips");

    // TODO: REMOVE BEFORE FLIGHT!!!!!!
    console::log_1(&JsValue::from_str("Initialized"));

    console::log_1(&JsValue::from_str(&response));

    Ok(())
}

// ref - wsPing(endpoint: string, message: string): Promise<string>
#[wasm_bindgen]
pub async fn ws_ping(
    endpoint: &str,
    message: &str,
) -> impl Into<JsValue> -> dyn Future<Output = Result<JsValue, JsValue>> { // TODO: resolve this
    // issue with impl and return type
    alert(&format!("wsPing is alive"));

    env_logger::init();

    // let it be hardcoded ip and port for cyrrent task
    let addr: SocketAddr = "127.0.0.1:4011"
        .to_string()
        .parse()
        .expect("Invalid address");

    // create tcp listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on {}", addr);

    while let Ok((stream, _remote_addr)) = listener.accept().await {
        // spawn new task for conn
        tokio::spawn(handle_conn(stream)); // TODO: send 2nd arg - &msg
    }
}

async fn handle_conn(stream: TcpStream) {
    // accept websocket conn
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during ws handshake: {}", e);
            return;
        }
    };

    // get tx and rx (sender and receiver)
    let (mut sender, mut receiver) = ws_stream.split();

    // handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Err(e) => {
                error!("Error processing msg: {}", e);
                break;
            }
            Ok(Message::Text(text)) => {
                // TODO handle msg the way I need
                let uppercased = text.to_uppercase();
                if let Err(e) = sender.send(Message::Text(uppercased)).await {
                    error!("Error while sending msg: {}", e);
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
        }
    }
}

// TODO: REMOVE BEFORE FLIGHT!!!!!!
//
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // #[test]
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
//
//     #[test]
//     fn endpoint_test() {
//         assert_eq!(ws_ping("/snacks", "Spicy chips!"), "SPICY CHIPS!");
//     }
// }
