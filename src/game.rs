use crate::input::Input;
use crate::graphics::spritebatch::SpriteBatch;


pub trait Game
{
    fn process_input(&mut self, input: &mut Input) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, graphics: &mut SpriteBatch) 
    {
    }
}