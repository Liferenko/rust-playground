use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (sender, receiver) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = sender.send(3) {
            println!("the receiver dropped");
        }
    });

    match receiver.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
