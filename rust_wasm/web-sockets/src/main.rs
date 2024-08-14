use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};
use std::net::SocketAddr;
use log::{info, error};



#[tokio::main]
async fn main() {
    env_logger::init();

    // let it be hardcoded ip and port for cyrrent task
    let addr: SocketAddr = "127.0.0.1:4011".to_string().parse().expect("Invalid address");

    // create tcp listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on {}", addr);

    while let Ok((stream, _remote_addr)) = listener.accept().await {
        // spawn new task for conn
        tokio::spawn(handle_conn(stream));
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

    // split ws => tx and rx (sender and receiver)
    let (mut sender, mut receiver) = ws_stream.split();

    // handle inc messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Err(e) => {
                error!("Error processing msg: {}", e);
                break;
            },
            Ok(Message::Text(text)) => {
                // TODO handle msg the way I need
                let uppercased = text.to_uppercase();
                if let Err(e) = sender.send(Message::Text(uppercased)).await {
                    error!("Error while sending msg: {}", e);
                }
            },
            Ok(Message::Close(_)) => break,
            Ok(_) => ()
        }
    }
}
