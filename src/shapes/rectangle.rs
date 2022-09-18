use crate::math::Vector2;

pub struct Rectangle
{
    pub width: f32,
    pub height: f32,

    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,

    pub position: Vector2<f32>,
    pub centre: Vector2<f32>
}

impl Rectangle
{
    pub fn new(position: Vector2<f32>, width: f32, height: f32) -> Self 
    {
        let left = position.x;
        let right = position.x + width;
        let top = position.y;
        let bottom = position.y + height;

        let centre = Vector2::new(position.x + (width / 2.0), position.y + (height / 2.0));

        Self { position, width, height, left, right, top, bottom, centre }
    }
}