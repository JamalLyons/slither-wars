use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tracing::debug;
use uuid::Uuid;

use super::create_random_position;
use super::food::Food;
use super::leaderboard::Leaderboard;
use super::snake::Snake;
use crate::constants::{Rgb, WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Clone, Debug)]
pub struct GameWorld
{
    pub snakes: Arc<Mutex<HashMap<Uuid, Snake>>>,
    pub width: f32,
    pub height: f32,
    pub food: Vec<Food>,
    pub total_snakes: u32,
    pub total_food: u32,
    pub leaderboard: Leaderboard,
}

impl GameWorld
{
    pub fn new() -> Self
    {
        let mut world = Self {
            snakes: Arc::new(Mutex::new(HashMap::new())),
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            food: Vec::new(),
            total_snakes: 0,
            total_food: 0,
            leaderboard: Leaderboard::new(),
        };

        world.spawn_food(50);
        world.spawn_bots(10);

        debug!("Game world initialized");

        world
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

    fn spawn_food(&mut self, amount: u32)
    {
        for _ in 0..amount {
            self.food.push(Food::new(create_random_position()));
        }

        self.total_food = self.food.len() as u32;

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

    // Other methods to manipulate the game world
}
