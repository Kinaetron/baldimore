
pub struct Rectangle
{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl Rectangle
{
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self 
    {
        let left = x;
        let right = x + width;
        let top = y;
        let bottom = y + height;

        Self { x, y, width, height, left, right, top, bottom }
    }
}