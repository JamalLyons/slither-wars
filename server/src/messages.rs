use serde::{Serialize, Deserialize};
use crate::game::{Player, Food, LeaderboardEntry};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    JoinGame {
        name: Option<String>,
    },
    PlayerInput {
        direction: f64,
        boosting: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    GameStateUpdate {
        players: Vec<Player>,
        food: Vec<Food>,
        leaderboard: Vec<LeaderboardEntry>,
    },
}
