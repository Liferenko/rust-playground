use mini_redis::{Connection, Frame}; // TODO: remove mini_redis and replace it with generic one
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let mut addr = String::new();
    addr.push_str("127.0.0.1");
    addr.push_str(":4011");

    let listener = TcpListener::bind(addr).await.unwrap();

    // TODO: add endpoint /wait-for-second-party/:unique-id
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new(); // TODO: use shared state
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            // Set(cmd) => Frame::Simple("ping".to_string()),
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                dbg!("req: {:?}", cmd);
                // TODO: wait for 2nd party's request
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                if let Some(value) = db.get(cmd.key().to_string().as_str()) {
                    println!("Seems like here I'll be waiting for 2nd party");
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("TODO. error: {:?}", cmd),
        };

        conn.write_frame(&response).await.unwrap();
    }
}

// #[cfg(test)]
// mod main_tests;
