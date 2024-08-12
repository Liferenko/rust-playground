#[test]
fn test_function() {
    // TODO:
    assert_eq!(2 + 2, 4);
}
#[tokio::main]
async fn main() {
    let mut client = Client::connect("localhost:4011").await.unwrap();

    client.set("foo", "bar".into()).await.unwrap();

    // Getting the value immediately works
    let val = client.get("foo").await.unwrap().unwrap();
    assert_eq!(val, "bssar");
}
