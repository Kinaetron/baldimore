use crate::game::Game;
use system_sdl::SDLSystem;
use crate::platform::system_sdl;
use crate::platform::graphics_device::GraphicsDevice;

pub fn run(window_title: &str, width: u32, height: u32, game: impl Game) -> Result<(), String>
{
    // Show logs from wgpu
    // env_logger::init();

    let sdl2_system = SDLSystem::new(window_title, width, height);
    let graphics_device = GraphicsDevice::new(&sdl2_system).unwrap();


    let mut is_running = true;

    while is_running
    {
        game.process_input();
        game.update();
        game.draw(&graphics_device);

        is_running = sdl2_system.keep_window_open();
    }

    Ok(())
}