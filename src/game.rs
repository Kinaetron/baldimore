use crate::input::keyboard::Keyboard;
use crate::graphics::spritebatch::SpriteBatch;


pub trait Game
{
    fn process_input(&mut self, keyboard: & Keyboard) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, sprite_batch: &mut SpriteBatch) 
    {
    }
}