use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Leaderboard
{
    pub player_ids: Vec<Uuid>,
    pub scores: Vec<u32>,
    pub player_names: Vec<String>,
}

impl Leaderboard
{
    pub fn new() -> Self
    {
        Self {
            player_ids: Vec::new(),
            scores: Vec::new(),
            player_names: Vec::new(),
        }
    }
}
