use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error};
use uuid::Uuid;

use super::create_random_position;
use super::food::Food;
use super::leaderboard::Leaderboard;
use super::snake::Snake;
use crate::constants::{Rgb, COLLISION_THRESHOLD, WORLD_HEIGHT, WORLD_WIDTH};
use crate::types::{ServerMessage, WsServerPacket};
use crate::ClientList;

#[derive(Clone, Debug)]
pub struct GameWorld
{
    pub snakes: Arc<Mutex<HashMap<Uuid, Snake>>>,
    pub width: f32,
    pub height: f32,
    pub foods: Arc<Mutex<Vec<Food>>>,
    pub total_snakes: u32,
    pub total_food: u32,
    pub leaderboard: Leaderboard,
    pub client_list: ClientList,
}

impl GameWorld
{
    pub fn new(client_list: ClientList) -> Self
    {
        let mut world = Self {
            snakes: Arc::new(Mutex::new(HashMap::new())),
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            foods: Arc::new(Mutex::new(vec![])),
            total_snakes: 0,
            total_food: 0,
            leaderboard: Leaderboard::new(),
            client_list,
        };

        world.spawn_food(50);
        world.spawn_bots(10);

        debug!("Game world initialized");

        world
    }

    pub fn update(&mut self) {
        let mut snakes = self.snakes.lock().unwrap();
        for snake in snakes.iter_mut() {
            snake.1.move_forward();
        }

        for snake in snakes.iter() {
            self.broadcast_message(
                ServerMessage::UpdateSnake,
                serde_json::json!(snake),
            ).unwrap_or_else(|e| {
                error!("Failed to broadcast snake update: {:?}", e);
            });
        }
    }

    pub fn handle_collisions(&mut self) {
        let mut snakes = self.snakes.lock().unwrap();
        let mut foods = self.foods.lock().unwrap();
    
        // Create a list of snake IDs to remove after checking collisions
        let mut snakes_to_remove = Vec::new();
    
        // Create a snapshot of the snakes for collision detection
        let snakes_snapshot: Vec<(Uuid, Vec<(f32, f32)>)> = snakes
            .iter()
            .map(|(id, snake)| (*id, snake.body.clone()))
            .collect();
    
        // Iterate over snakes mutably to update their state
        for (snake_id, snake) in snakes.iter_mut() {
            // Check collision with foods
            let mut i = 0;
            while i < foods.len() {
                let food_position = foods[i].position;
                let food_value = foods[i].value;
    
                if self.check_collision(snake.position, food_position) {
                    snake.grow(1);
                    snake.score += food_value;
                    // Remove food from the game
                    foods.remove(i);
                    self.total_food = foods.len() as u32;
    
                    // Broadcast updated snake data
                    self.broadcast_message(
                        ServerMessage::UpdateSnake,
                        serde_json::json!(snake),
                    )
                    .unwrap_or_else(|e| {
                        error!("Failed to broadcast snake update: {:?}", e);
                    });
    
                    // Broadcast that the food was eaten
                    self.broadcast_message(
                        ServerMessage::FoodEaten,
                        serde_json::json!({ "position": food_position }),
                    )
                    .unwrap_or_else(|e| {
                        error!("Failed to broadcast food eaten: {:?}", e);
                    });
                    // Don't increment i because we've removed an item
                } else {
                    i += 1;
                }
            }
    
            // Check collision with other snakes using the snapshot
            for (other_id, other_body) in &snakes_snapshot {
                // Skip checking collision with self
                if snake_id == other_id {
                    continue;
                }
    
                // Check if the snake's head collides with any segment of the other snake's body
                for segment in other_body {
                    if self.check_collision(snake.position, *segment) {
                        // Handle snake-to-snake collision
                        snake.is_dead = true;
                        snakes_to_remove.push(*snake_id);
    
                        // Broadcast that the snake has died
                        self.broadcast_message(
                            ServerMessage::SnakeDied,
                            serde_json::json!({ "id": snake_id }),
                        )
                        .unwrap_or_else(|e| {
                            error!("Failed to broadcast snake death: {:?}", e);
                        });
    
                        break;
                    }
                }
                if snake.is_dead {
                    break; // No need to check further if snake is already dead
                }
            }
        }
    
        // Remove dead snakes from the game
        for id in snakes_to_remove {
            if let Some(dead_snake) = snakes.remove(&id) {
                self.total_snakes = snakes.len() as u32;
    
                // Drop food at the dead snake's body positions
                for segment in dead_snake.body.clone() {
                    foods.push(Food::new(segment, Some(dead_snake.color)));
                }
                self.total_food = foods.len() as u32;
    
                // Broadcast the new food positions
                self.broadcast_message(
                    ServerMessage::FoodSpawned,
                    serde_json::json!({ "positions": dead_snake.body, "color": dead_snake.color }),
                )
                .unwrap_or_else(|e| {
                    error!("Failed to broadcast new food positions: {:?}", e);
                });
            }
        }
    }
    
    
    
    fn check_collision(&self, pos1: (f32, f32), pos2: (f32, f32)) -> bool {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;
        let distance = (dx * dx + dy * dy).sqrt();
        let collision_threshold = COLLISION_THRESHOLD;
        distance < collision_threshold
    }

    pub fn add_snake(&mut self, snake: Snake)
    {
        let mut snakes = self.snakes.lock().unwrap();
        snakes.insert(snake.id, snake);
        self.total_snakes = snakes.len() as u32;
    }

    pub fn remove_snake(&mut self, id: Uuid)
    {
        let mut snakes = self.snakes.lock().unwrap();
        snakes.remove(&id);
        self.total_snakes = snakes.len() as u32;
    }

    pub fn update_snake_direction(&mut self, snake_id: Uuid, direction: f32) {
        let mut snakes = self.snakes.lock().unwrap();
        if let Some(snake) = snakes.get_mut(&snake_id) {
            snake.direction = direction;
        }
    }

    fn spawn_food(&mut self, amount: u32) {
        let mut foods = self.foods.lock().unwrap();
        for _ in 0..amount {
            foods.push(Food::new(create_random_position(), None));
        }
    
        self.total_food = foods.len() as u32;
    
        debug!("Food spawned");
    }

    fn spawn_bots(&mut self, amount: u32)
    {
        let mut snakes = self.snakes.lock().unwrap();
        snakes.extend((0..amount).map(|_| {
            let name = format!("Bot {}", amount - 1);

            (
                Uuid::new_v4(),
                Snake::new(Uuid::new_v4(), name, Rgb::random(), true, create_random_position()),
            )
        }));

        self.total_snakes = snakes.len() as u32;

        debug!("Bots spawned");
    }

    pub fn to_string(&self) -> String
    {
        format!(
            "Width: {}, Height: {}, Snakes: {}, Food: {}",
            self.width, self.height, self.total_snakes, self.total_food
        )
    }

    pub fn broadcast_message(
        &self,
        message_type: ServerMessage,
        data: serde_json::Value,
    ) -> anyhow::Result<()> {
        let clients = self.client_list.lock().unwrap();
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
}
