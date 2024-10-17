// game.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;
use uuid::Uuid;

// Constants for game settings
const WORLD_WIDTH: i32 = 1000;
const WORLD_HEIGHT: i32 = 1000;
const MAX_FOOD_COUNT: usize = 100;
const PLAYER_BASE_SPEED: f64 = 2.0;
const PLAYER_BOOST_MULTIPLIER: f64 = 1.5;
const PLAYER_DEFAULT_LENGTH: usize = 10;
const PLAYER_MAX_LENGTH: usize = 100; // Maximum length a player can reach
const PLAYER_BASE_WIDTH: f64 = 5.0;
const PLAYER_MAX_WIDTH: f64 = 20.0;
const PLAYER_GROWTH_RATE: f64 = 0.1; // Growth per food item
const PLAYER_SPEED_DECAY_RATE: f64 = 0.01; // Speed decrease per unit length
const PLAYER_MIN_SCORE: i32 = 10; // Minimum score a player can have
const BOOST_POINT_COST: i32 = 1; // Points deducted when boosting
const BOOST_TICK_INTERVAL: u32 = 20; // Ticks per point deduction (assuming 20 ticks per second)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub players: HashMap<Uuid, Player>,
    pub food: Vec<Food>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            food: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player_id: Uuid, name: Option<String>) {
        let mut rng = rand::thread_rng();
        let position = (
            rng.gen_range(0..WORLD_WIDTH),
            rng.gen_range(0..WORLD_HEIGHT),
        );
        let color = (
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        );
        let player = Player {
            id: player_id,
            name,
            position,
            direction: 0.0,
            length: PLAYER_DEFAULT_LENGTH as f64,
            target_length: PLAYER_DEFAULT_LENGTH as f64,
            width: PLAYER_BASE_WIDTH,
            boosting: false,
            color,
            score: PLAYER_MIN_SCORE, // Start with 10 points
            alive: true,
            growth_remaining: 0.0,
            boosting_tick_counter: 0,
        };
        self.players.insert(player_id, player);
    }

    pub fn remove_player(&mut self, player_id: Uuid) {
        self.players.remove(&player_id);
    }

    pub fn update_player_input(&mut self, player_id: Uuid, direction: f64, boosting: bool) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.direction = direction;
            // Prevent boosting if score is at minimum
            if player.score > PLAYER_MIN_SCORE {
                player.boosting = boosting;
            } else {
                player.boosting = false;
            }
        }
    }

    pub fn update(&mut self) {
        // Update player positions and growth
        for player in self.players.values_mut() {
            if player.alive {
                player.update();
            }
        }

        // Handle collisions
        self.handle_collisions();

        // Spawn food if needed
        self.spawn_food();
    }

    fn handle_collisions(&mut self) {
        // Simplified collision detection
        let player_positions: Vec<(Uuid, (i32, i32), f64)> = self
            .players
            .iter()
            .map(|(id, player)| (*id, player.position, player.width))
            .collect();

        for (id, player) in self.players.iter_mut() {
            // Check collision with food
            self.food.retain(|food| {
                let distance = distance(player.position, food.position);
                if distance < (player.width / 2.0) + (food.size / 2.0) {
                    player.score += 1;
                    // Increase target_length but cap it at PLAYER_MAX_LENGTH
                    let new_target_length = player.target_length + PLAYER_GROWTH_RATE;
                    if new_target_length <= PLAYER_MAX_LENGTH as f64 {
                        player.target_length = new_target_length;
                        player.growth_remaining += PLAYER_GROWTH_RATE;
                    } else {
                        player.target_length = PLAYER_MAX_LENGTH as f64;
                    }
                    false // Remove the food
                } else {
                    true
                }
            });

            // Check collision with other players
            for (other_id, position, width) in &player_positions {
                if id != other_id {
                    let distance = distance(player.position, *position);
                    if distance < (player.width / 2.0) + (width / 2.0) {
                        player.alive = false;
                        break;
                    }
                }
            }
        }

        // Remove dead players
        self.players.retain(|_, player| player.alive);
    }

    fn spawn_food(&mut self) {
        if self.food.len() < MAX_FOOD_COUNT {
            let mut rng = rand::thread_rng();
            let position = (
                rng.gen_range(0..WORLD_WIDTH),
                rng.gen_range(0..WORLD_HEIGHT),
            );
            let color = (
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            );
            let size = 5.0; // Fixed size for food
            self.food.push(Food { position, color, size });
        }
    }

    pub fn get_leaderboard(&self) -> Vec<LeaderboardEntry> {
        let mut leaderboard: Vec<LeaderboardEntry> = self
            .players
            .values()
            .map(|player| LeaderboardEntry {
                id: player.id,
                name: player.name.clone(),
                score: player.score,
            })
            .collect();

        leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
        leaderboard.truncate(10);
        leaderboard
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: Option<String>,
    pub position: (i32, i32),
    pub direction: f64, // Angle in degrees
    pub length: f64,    // Current length
    pub target_length: f64, // Length player is growing towards
    pub width: f64,     // Width of the snake
    pub boosting: bool,
    pub color: (u8, u8, u8),
    pub score: i32,
    pub alive: bool,
    pub growth_remaining: f64, // Amount of growth left to process
    pub boosting_tick_counter: u32, // Counts ticks while boosting
}

impl Player {
    pub fn update(&mut self) {
        // Handle boosting logic
        if self.score <= PLAYER_MIN_SCORE {
            self.boosting = false;
            self.boosting_tick_counter = 0;
        }

        if self.boosting {
            self.boosting_tick_counter += 1;
            // Deduct points every BOOST_TICK_INTERVAL ticks
            if self.boosting_tick_counter >= BOOST_TICK_INTERVAL {
                if self.score > PLAYER_MIN_SCORE {
                    self.score -= BOOST_POINT_COST;
                    // Ensure score doesn't go below minimum
                    if self.score < PLAYER_MIN_SCORE {
                        self.score = PLAYER_MIN_SCORE;
                        self.boosting = false;
                    }
                } else {
                    self.boosting = false;
                }
                self.boosting_tick_counter = 0;
            }
        } else {
            self.boosting_tick_counter = 0;
        }

        self.move_forward();
        self.process_growth();
        self.update_width();
    }

    fn move_forward(&mut self) {
        let base_speed = PLAYER_BASE_SPEED;
        let speed_decay = self.length * PLAYER_SPEED_DECAY_RATE;
        let mut speed = base_speed - speed_decay;
        if speed < 0.5 {
            speed = 0.5; // Minimum speed
        }
        if self.boosting {
            speed *= PLAYER_BOOST_MULTIPLIER;
        }

        let rad = self.direction.to_radians();
        self.position.0 += (rad.cos() * speed) as i32;
        self.position.1 += (rad.sin() * speed) as i32;

        // Wrap around the game world boundaries
        self.position.0 = (self.position.0 + WORLD_WIDTH) % WORLD_WIDTH;
        self.position.1 = (self.position.1 + WORLD_HEIGHT) % WORLD_HEIGHT;
    }

    fn process_growth(&mut self) {
        if self.length < self.target_length {
            let growth_step = 0.05; // Adjust this for smoother or faster growth
            self.length += growth_step;
            if self.length > self.target_length {
                self.length = self.target_length;
            }
            // Ensure length doesn't exceed PLAYER_MAX_LENGTH
            if self.length > PLAYER_MAX_LENGTH as f64 {
                self.length = PLAYER_MAX_LENGTH as f64;
            }
            // Ensure length doesn't go below default length
            if self.length < PLAYER_DEFAULT_LENGTH as f64 {
                self.length = PLAYER_DEFAULT_LENGTH as f64;
            }
        }
    }

    fn update_width(&mut self) {
        let width_increase = (self.length - PLAYER_DEFAULT_LENGTH as f64) * 0.1;
        self.width = PLAYER_BASE_WIDTH + width_increase;
        if self.width > PLAYER_MAX_WIDTH {
            self.width = PLAYER_MAX_WIDTH;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Food {
    pub position: (i32, i32),
    pub color: (u8, u8, u8),
    pub size: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardEntry {
    pub id: Uuid,
    pub name: Option<String>,
    pub score: i32,
}

// Utility function to calculate distance between two points
fn distance(a: (i32, i32), b: (i32, i32)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    ((dx * dx + dy * dy) as f64).sqrt()
}
