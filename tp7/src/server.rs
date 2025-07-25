// TP 7: Serveur et Client WebSocket

// Description:
// Ce TP vise à implémenter un serveur et un client WebSocket en Rust. Les WebSockets permettent une communication bidirectionnelle persistante entre un client et un serveur, idéale pour les applications en temps réel comme les chats ou les tableaux de bord interactifs. Les étudiants utiliseront une crate comme tokio-tungstenite.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Serveur WebSocket lancé sur ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut ws_stream = accept_async(stream).await.unwrap();
            println!("Nouveau client connecté!");

            while let Some(msg) = ws_stream.next().await {
                let msg = msg.unwrap();
                println!("Message reçu: {}", msg);
                ws_stream.send(msg).await.unwrap();
            }
        });
    }
}