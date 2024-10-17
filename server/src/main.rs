use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info};

mod constants;
mod errors;
mod game;
mod server;
mod types;

type Tx = UnboundedSender<Message>;
type ClientList = Arc<Mutex<Vec<Tx>>>;

#[tokio::main]
async fn main()
{
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    info!("Starting slither.io server...");

    let client_list: ClientList = Arc::new(Mutex::new(Vec::new()));

    // Initialize game world
    let game_world = game::world::GameWorld::new();

    debug!("{:?}", game_world.to_string());

    // Start the server
    if let Err(e) = server::start_server(game_world, client_list).await {
        eprintln!("Server error: {:?}", e);
    }
}
