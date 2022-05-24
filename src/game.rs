use crate::graphics::spritebatch::SpriteBatch;


pub trait Game
{
    fn process_input(&self) {
    }

    fn update(&self) {
    }

    fn draw(&self, sprite_batch: &mut SpriteBatch) 
    {
    }
}