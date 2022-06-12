use crate::input::Input;
use crate::graphics::draw::Draw;


pub trait Game
{
    fn process_input(&mut self, input: &mut Input) {
    }

    fn update(&mut self) {
    }

    fn draw(&mut self, draw: &mut Draw) 
    {
    }
}