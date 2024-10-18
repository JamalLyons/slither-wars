use crate::constants::Rgb;

#[derive(Clone, Debug)]
pub struct Food
{
    pub position: (f32, f32),
    pub value: u32,
    pub color: Rgb,
}

impl Food
{
    pub fn new(position: (f32, f32), color: Option<Rgb>) -> Self
    {
        Self {
            position,
            value: 1,
            color: color.unwrap_or(Rgb::random_food()),
        }
    }

    // Methods related to food
}
