use mini_redis::{Connection, Frame}; // TODO: remove mini_redis and replace it with generic one
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let mut addr = String::new();
    addr.push_str("127.0.0.1");
    addr.push_str(":4011");

    let listener = TcpListener::bind(addr).await.unwrap();

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

    let mut db = HashMap::new();
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("req: {:?}", cmd);
                println!("expire in {:?}", cmd.expire());
                // TODO: wait for 2nd party's request
                Frame::Simple("wait for 2nd party's request".to_string())
            }

            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
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
