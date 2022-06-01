use crate::graphics::spritebatch::SpriteBatch;
use crate::input::{gamepad::Gamepad, keyboard::Keyboard, mouse::Mouse};


pub trait Game
{
    fn process_input(&mut self,gamepad: &mut Gamepad,  keyboard: &mut Keyboard, mouse: &mut Mouse) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, sprite_batch: &mut SpriteBatch) 
    {
    }
}