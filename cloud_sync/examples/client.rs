use mini_redis::{client, Result}; // TODO: remove mini_redis and replace it with generic one
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut addr = String::new();
    addr.push_str("127.0.0.1");
    addr.push_str(":4011");

    let mut client = client::connect(addr).await?;

    // read args
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let msg_arg = &args[1].to_uppercase();

        println!("ACTION: {:?}", &msg_arg);
        client.set(&msg_arg, ">> pong << TODO".into()).await?;

        // Timeout for waiting 2nd party's request
        // TODO: seems like it belongs to server side
        // tokio::time::sleep(Duration::from_millis(4000)).await;

        // Get a response from server
        let result = client.get(&msg_arg).await?; // TODO: response when 2nd party sent a request
        println!("response: {:?}", result);
    } else {
        println!("No args provided");
    }

    // TODO: waiting for 2nd party request andalso time expires
    // Let 1st party know we're waiting for 2nd party's request
    // {
    //     println!("server will response as soon as 2nd party will initiate request")
    // }

    Ok(())
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
