use crate::graphics::spritebatch::SpriteBatch;


pub trait Game
{
    fn process_input(&mut self) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, sprite_batch: &mut SpriteBatch) 
    {
    }
}