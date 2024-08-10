use mini_redis::{client, Result}; // TODO: remove mini_redis and replace it with generic one

#[tokio::main]
async fn main() -> Result<()> {
    let mut addr = String::new();
    addr.push_str("127.0.0.1");
    addr.push_str(":4011");

    let mut client = client::connect(addr).await?;

    client
        .set("request", ">> ping << TODO: add req msg".into())
        .await?;

    // Let 1st party know we're waiting for 2nd party's request
    // {
    //     println!("server will response as soon as 2nd party will initiate request")
    // }

    // Get a response from server
    let result = client.get("pong").await?; // TODO: response when 2nd party sent a request
    println!("response: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod client_test;
