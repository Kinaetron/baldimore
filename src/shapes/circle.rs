use crate::math::Vector2;

pub struct Circle
{
    pub radius: f32,
    pub centre: Vector2<f32>,
}

impl Circle 
{
    pub fn new(position: Vector2<f32>, radius: f32) -> Self {
        Self { centre: position, radius }
    }    
}