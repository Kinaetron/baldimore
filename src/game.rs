use crate::graphics::spritebatch::SpriteBatch;
use crate::input::{keyboard::Keyboard, mouse::Mouse};


pub trait Game
{
    fn process_input(&mut self, keyboard: &Keyboard, mouse: &Mouse) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, sprite_batch: &mut SpriteBatch) 
    {
    }
}