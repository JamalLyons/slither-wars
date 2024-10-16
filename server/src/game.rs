use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    players: HashMap<u32, Player>,
    food: Vec<Food>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Food {
    position: (i32, i32),
    color: (u8, u8, u8),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    score: i32,
    position: (i32, i32),
    boosting: bool,
    color: (u8, u8, u8),
}

impl GameState {
    pub fn update_player_position(&mut self, player_id: u32, position: (i32, i32)) {
        let p = match self.players.get_mut(&player_id) {
            Some(p) => p,
            None => return,
        };

        p.position = position;
    }

    pub fn update_player_score(&mut self, player_id: u32, score: i32) {
        let p = match self.players.get_mut(&player_id) {
            Some(p) => p,
            None => return,
        };

        p.score = score;
    }

    pub fn generate_food(&mut self, food: Food) {
        self.food.push(food);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            players: HashMap::new(),
            food: Vec::new(),
        }
    }
    
}