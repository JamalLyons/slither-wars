use anyhow::Result;
use futures::SinkExt;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::constants::Rgb;
use crate::game::create_random_position;
use crate::game::snake::Snake;
use crate::game::world::GameWorld;
use crate::types::{ClientMessage, ServerMessage, WsClientPacket, WsServerPacket};
use crate::{ClientList, Tx};

pub async fn start_websocket_server(game_world: GameWorld, client_list: ClientList) -> Result<()>
{
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket server listening on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New client connection from: {}", addr);
        let game_world = game_world.clone();
        let client_list = client_list.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, game_world, client_list).await {
                error!("Error handling connection: {:?}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, mut game_world: GameWorld, client_list: ClientList) -> Result<()>
{
    let ws_stream = accept_async(stream).await?;
    let (write, mut read) = ws_stream.split();

    // Set up an mpsc channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Add the sender to the client list
    {
        let mut clients = client_list.lock().unwrap();
        clients.push(tx.clone());
    }

    //Initialize player_id as None
    let mut player_id: Option<Uuid> = None;

    // Spawn a task to forward messages from rx to write
    tokio::spawn(async move {
        let mut write = write;
        while let Some(message) = rx.recv().await {
            if let Err(e) = write.send(message).await {
                error!("Error sending message to client: {:?}", e);
                break;
            }
        }
    });

    // Process incoming messages
    while let Some(msg) = read.next().await {
        let msg = msg?;

        // Ignore non-text messages
        if !msg.is_text() {
            continue;
        }

        let packet: WsClientPacket = serde_json::from_str(&msg.to_string())?;

        handle_client_packet(
            &mut game_world,
            tx.clone(),
            client_list.clone(),
            packet,
            &mut player_id, // Pass mutable reference
        )
        .await?;
    }

    // After the read loop ends (client disconnects)
    // Remove the sender from the client list upon disconnection
    {
        let mut clients = client_list.lock().unwrap();
        clients.retain(|client_tx| !client_tx.same_channel(&tx));
    }

    // Broadcast PlayerLeft event if player_id is known
    if let Some(id) = player_id {
        // Remove the player from the game world
        game_world.remove_snake(id);

        // Broadcast the PlayerLeft event
        broadcast_message(&client_list, ServerMessage::PlayerLeft, serde_json::json!({ "id": id }))?;

        debug!("Removed player {} from the game", id);
    } else {
        warn!("No player ID found for disconnected client");
    }

    Ok(())
}

async fn handle_client_packet(
    game_world: &mut GameWorld,
    tx: Tx,
    client_list: ClientList,
    packet: WsClientPacket,
    player_id: &mut Option<Uuid>,
) -> Result<()>
{
    match packet.message {
        ClientMessage::JoinGame => {
            let name = packet.data.to_string();

            let snake = Snake::new(Uuid::new_v4(), name.clone(), Rgb::random(), false, create_random_position());

            // Update player_id via mutable reference
            // This way we can track the player's ID in the client list
            *player_id = Some(snake.id);

            game_world.add_snake(snake.clone());

            debug!("Added new player {} to the game", name);

            tx.send(Message::Text(serde_json::to_string(&WsServerPacket {
                message: ServerMessage::PlayerInit,
                data: serde_json::json!(snake),
            })?))?;

            broadcast_message(&client_list, ServerMessage::PlayerJoined, serde_json::json!(snake))?;
        }
        ClientMessage::MoveSnake => {
            let direction: f32 = serde_json::from_value(packet.data)?;

            if let Some(id) = player_id {
                game_world.update_snake_direction(*id, direction);
            }
        }
        _ => {}
    }

    Ok(())
}

fn broadcast_message(client_list: &ClientList, message_type: ServerMessage, data: serde_json::Value) -> Result<()>
{
    let clients = client_list.lock().unwrap();
    let response_packet = WsServerPacket {
        message: message_type,
        data,
    };

    let response_text = serde_json::to_string(&response_packet)?;

    for tx in clients.iter() {
        tx.send(Message::Text(response_text.clone())).map_err(|e| {
            error!("Failed to send message: {:?}", e);
            anyhow::anyhow!("Failed to send message")
        })?;
    }

    Ok(())
}
