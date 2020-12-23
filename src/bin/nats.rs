use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    // Initialize message queue client
    let nc = nats::Options::new()
        .connect_async("127.0.0.1:4222")
        .await
        .expect("Cannot connect to NATS server");

    let mut sub = nc
        .queue_subscribe("request_subject", "rust-box")
        .await
        .unwrap();

    while let Some(msg) = sub.next().await {
        tokio::spawn(async move {
            msg.respond("").await.unwrap();
        });
    }
}
