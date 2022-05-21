use crate::platform::graphics_device::GraphicsDevice;


pub trait Game
{
    fn process_input(&self) {
    }

    fn update(&self) {
    }

    fn draw(&self, graphics_device: &GraphicsDevice) {
    }
}