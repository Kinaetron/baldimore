use std::sync::Arc;
use crate::math::Vector2;
use crate::rectangle::Rectangle;
use crate::graphics::{texture::Texture, draw::Draw, colour::Colour};

pub struct Animation
{
    frame: u32,
    column: u32,
    frame_time: u32,
    is_looping: bool,
    frame_count: u32,
    frame_timer: u32,
    texture: Arc<Texture>,
    cell_size: Vector2<f32>,
}

impl Animation
{
    pub fn new(texture: Arc<Texture>, column: u32, frame_time: u32, frame_count: u32, cell_size: Vector2<f32>, is_looping: bool) -> Self {
        Self { frame: 0, frame_timer: 0, column, frame_time, frame_count, is_looping, texture, cell_size }
    }

    pub fn update(&mut self)
    {
        if self.frame_timer == self.frame_time
        {
            self.frame_timer = 0;
            self.frame += 1;
        }

        if self.frame == self.frame_count 
        {
            if self.is_looping {
                self.frame = 0;
            }
            else {
                self.frame = self.frame_count - 1;
            }
        }

        self.frame_timer += 1;
    }

    pub fn draw(&mut self, position: Vector2<f32>, rotation: f32, colour: Colour, draw: &mut Draw)
    {
        let draw_area = Rectangle::new(self.frame as f32 * self.cell_size.x, self.column as f32, self.cell_size.x, self.cell_size.y);
        draw.sprite(Arc::clone(&self.texture), position, &draw_area, self.cell_size, rotation, colour);
    }
}