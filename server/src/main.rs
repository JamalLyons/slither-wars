use tokio::{net::TcpListener, sync::mpsc};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::{Mutex, broadcast};
use tracing::{info, error};
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message as WsMessage;

mod game;
mod messages;

use game::GameState;
use messages::{ClientMessage, ServerMessage};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await?;
    info!("Server is listening on {}", addr);

    // Shared game state
    let game_state = Arc::new(Mutex::new(GameState::new()));

    // Create a broadcast channel for sending messages to all clients
    let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(16);

    // Clone game state and broadcast_tx for the game loop
    let game_state_clone = Arc::clone(&game_state);
    let broadcast_tx_clone = broadcast_tx.clone();

    // Start the game loop
    tokio::spawn(game_loop(game_state_clone, broadcast_tx_clone));

    // Accept incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        let game_state = Arc::clone(&game_state);
        let broadcast_tx = broadcast_tx.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, game_state, broadcast_tx).await {
                error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

async fn game_loop(
    game_state: Arc<Mutex<GameState>>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
) {
    let tick_rate = tokio::time::Duration::from_millis(50); // 20 ticks per second

    let mut interval = tokio::time::interval(tick_rate);

    loop {
        interval.tick().await;

        let mut state = game_state.lock().await;

        // Update game state (move players, handle collisions, spawn food, etc.)
        state.update();

        // Prepare the update message
        let update_message = ServerMessage::GameStateUpdate {
            players: state.players.values().cloned().collect(),
            food: state.food.clone(),
            leaderboard: state.get_leaderboard(),
        };

        // Broadcast the update to all connected clients
        let _ = broadcast_tx.send(update_message);
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    game_state: Arc<Mutex<GameState>>,
    broadcast_tx: broadcast::Sender<ServerMessage>,
) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;

    let (write, mut read) = ws_stream.split();

    // Generate a unique player ID
    let player_id = uuid::Uuid::new_v4();

    // Subscribe to the broadcast channel
    let mut broadcast_rx = broadcast_tx.subscribe();

    // Create a channel to signal the write task to shut down
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel::<()>();

    // Clone player_name for use in the write task
    let player_name_clone = Arc::new(Mutex::new(None::<String>));

    // Spawn a task to forward broadcast messages to this client
    let write_task = {
        let player_name_clone = Arc::clone(&player_name_clone);
        tokio::spawn(async move {
            let mut write = write;
            loop {
                tokio::select! {
                    // Receive messages from the broadcast channel
                    result = broadcast_rx.recv() => {
                        match result {
                            Ok(message) => {
                                let msg_text = serde_json::to_string(&message).unwrap();
                                if let Err(e) = write.send(WsMessage::Text(msg_text)).await {
                                    // Log and break if sending fails
                                    info!("Client {:?} disconnected while sending message: {}", *player_name_clone.lock().await, e);
                                    break;
                                }
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => {
                                // Handle lagged messages if necessary
                            }
                            Err(_) => {
                                // Broadcast channel closed
                                break;
                            }
                        }
                    }
                    // Receive shutdown signal
                    _ = shutdown_rx.recv() => {
                        // Shutdown signal received
                        break;
                    }
                }
            }
        })
    };

    // Wait for the client to send a JoinGame message
    let mut player_name: Option<String> = None;
    while let Some(msg) = read.next().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                // Deserialize the client message
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::JoinGame { name }) => {
                        player_name = name.clone();
                        // Update the player_name in the write task
                        *player_name_clone.lock().await = name.clone();
                        // Add player to the game state
                        {
                            let mut state = game_state.lock().await;
                            state.add_player(player_id, player_name.clone());
                        }
                        info!("Player {} connected with name {:?}", player_id, player_name);
                        break; // Exit the loop to proceed with normal message handling
                    }
                    _ => {
                        error!("Expected JoinGame message, received: {}", text);
                        // Optionally, send an error message to the client
                    }
                }
            }
            Ok(WsMessage::Close(_)) => {
                info!("Connection closed before joining");
                return Ok(());
            }
            _ => {
                // Ignore other messages until the player joins
            }
        }
    }

    // Main loop to handle messages from the client
    while let Some(msg) = read.next().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                // Deserialize the client message
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => {
                        match client_msg {
                            ClientMessage::PlayerInput { direction, boosting } => {
                                let mut state = game_state.lock().await;
                                state.update_player_input(player_id, direction, boosting);
                            }
                            _ => {
                                error!("Unexpected message: {:?}", client_msg);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse client message: {}", e);
                    }
                }
            }
            Ok(WsMessage::Close(_)) => {
                info!("Player {:?} disconnected", player_name);
                // Remove player from game state
                {
                    let mut state = game_state.lock().await;
                    state.remove_player(player_id);
                }
                // Send shutdown signal to the write task
                let _ = shutdown_tx.send(());
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Remove player from game state if loop exits unexpectedly
    {
        let mut state = game_state.lock().await;
        state.remove_player(player_id);
    }
    // Send shutdown signal to the write task
    let _ = shutdown_tx.send(());

    // Wait for the write task to finish
    let _ = write_task.await;

    info!("Player {:?} disconnected", player_name);

    Ok(())
}