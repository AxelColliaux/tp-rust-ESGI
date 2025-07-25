use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080";
    let (mut ws_stream, _) = connect_async(url).await.unwrap();

    ws_stream.send("Hello serveur!".into()).await.unwrap();
    if let Some(msg) = ws_stream.next().await {
        println!("Réponse du serveur: {}", msg.unwrap());
    }
}