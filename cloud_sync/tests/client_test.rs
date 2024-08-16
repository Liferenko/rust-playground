use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_post_request() {
    let client = Client::new();
    let url = "http://127.0.0.1:4012/wait-for-second-party/999";

    // Send the first request
    let first_response = client
        .post(url)
        .send()
        .await
        .expect("Failed to send first request");
    println!("First request response: {:?}", first_response);

    // Wait for 2 seconds
    sleep(Duration::from_secs(2)).await;

    // Send the second request
    let second_response = client
        .post(url)
        .send()
        .await
        .expect("Failed to send second request");
    println!("Second request response: {:?}", second_response);
}
