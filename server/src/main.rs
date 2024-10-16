use tracing::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tracing_subscriber::FmtSubscriber;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum Action {
    GameInit,
    GameLoad,
    GameUpdate,
    GameOver,
}

#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    action: Action,
}

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

    info!("Connection established");

    // Split the WebSocket into a sender and receiver
    let (mut write, mut read) = ws_stream.split();

    // Echo incoming messages back to the client
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received: {:?}", text);
            }
            Ok(Message::Binary(bin)) => {
                // Convert the binary data back to a string
                match String::from_utf8(bin) {
                    Ok(json_string) => {
                        // Attempt to deserialize the JSON string into the Payload struct
                        match serde_json::from_str::<Payload>(&json_string) {
                            Ok(payload) => {
                                //? do something with the actions
                                match payload.action {
                                    Action::GameInit => {
                                        info!("Game Init action received");
                                        
                                        // Send a response back to the client
                                        let response = serde_json::to_vec(&Payload { action: Action::GameLoad }).unwrap();
                                        write.send(Message::Binary(response)).await.unwrap();
                                    },
                                    Action::GameLoad => {}
                                    Action::GameUpdate => {
                                        info!("Game Update action received");
                                    }
                                    Action::GameOver => {
                                        info!("Game Over action received");
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize JSON: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to convert binary to string: {}", e);
                    }
                }
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
            Err(e) => {
                error!("Error (read.next): {}", e);
                break;
            }
            _ => (),
        }
    }
}
