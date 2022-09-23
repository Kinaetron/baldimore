use crate::math::Vector2;

pub struct Rectangle
{
    pub width: f32,
    pub height: f32,
    
    pub position: Vector2<f32>,
}

impl Rectangle
{
    pub fn left(&self) -> f32 {
        self.position.x 
    }

    pub fn right(&self)  -> f32 {
        self.position.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.position.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + self.height
    }

    pub fn centre(&self) -> Vector2<f32> {
        Vector2::new(self.position.x + (self.width / 2.0), self.position.y + (self.height / 2.0))
    }

    pub fn new(position: Vector2<f32>, width: f32, height: f32) -> Self  {
        Self { position, width, height }
    }
}