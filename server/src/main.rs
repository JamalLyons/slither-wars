use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let addr = "127.0.0.1:9001".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("WebSocket server listening on {}", addr);

    // Accept incoming TCP connections
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

// Handle each WebSocket connection
async fn handle_connection(stream: TcpStream) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("Error (accept_async): {}", e);
            return;
        }
    };

    info!("New WebSocket connection established");

    // Split the WebSocket into a sender and receiver
    let (mut write, mut read) = ws_stream.split();

    // Echo incoming messages back to the client
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received: {}", text);
                // Echo the message back
                write.send(Message::Text(text)).await.expect("Failed to send message");
            }
            Ok(Message::Binary(bin)) => {
                info!("Received: {:?}", bin);
                // Echo the message back
                write.send(Message::Binary(bin)).await.expect("Failed to send message");
            }
            Ok(Message::Ping(_)) => {
                info!("Ping received");
            }
            Ok(Message::Pong(_)) => {
                info!("Pong received");
            }
            Ok(Message::Close(_)) => {
                info!("Connection closed");
                break;
            }
            _ => (),
        }
    }
}
