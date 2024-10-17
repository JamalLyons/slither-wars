pub mod websocket;

use anyhow::Result;

use crate::game::world::GameWorld;
use crate::ClientList;

pub async fn start_server(game_world: GameWorld, client_list: ClientList) -> Result<()>
{
    websocket::start_websocket_server(game_world, client_list).await
}
