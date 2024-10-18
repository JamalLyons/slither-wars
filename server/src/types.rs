use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct WsClientPacket
{
    /// The message that is sent to the server
    pub message: ClientMessage,
    pub data: Value,
}

/// Events sent from the client to the server
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage
{
    /// Game join event. Triggered after the 'InitPlayer'. This event sends the server the players name.
    JoinGame,
    /// Game leave event. Triggered when the player disconnects from the game.
    LeaveGame,
    MoveSnake,
    Pong,
    FoodEat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WsServerPacket
{
    pub message: ServerMessage,
    pub data: Value,
}

/// Events created by the server sent to a/all clients
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage
{
    /// Event triggered when a new player joins.
    /// This data is sent directly to the client. We need to let the client know 
    /// which player it is before we can manage all players.
    PlayerInit,
    /// Event triggered when a new player joins the game. (Used to alert other players of new player)
    PlayerJoined,
    /// Event triggered when a player disconnects from the game. (Used to alert other players of disconnect)
    PlayerLeft,
    SnakeDied,

    UpdateSnake,

    IncreasePlayerLength,
    DecreasePlayerLength,

    UpdateLeaderboard,
    UpdateMinimap,
    FoodSpawned,
    FoodEaten,
}
