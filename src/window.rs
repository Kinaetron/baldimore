use crate::game::Game;
use system_sdl::SDLSystem;
use crate::platform::system_sdl;
use crate::platform::graphics_device::GraphicsDevice;

use std::time::Instant;

pub fn run(window_title: &str, width: u32, height: u32, game: impl Game) -> Result<(), String>
{
    // Show logs from wgpu
    // env_logger::init();

    let sdl2_system = SDLSystem::new(window_title, width, height);
    let graphics_device = GraphicsDevice::new(&sdl2_system).unwrap();

    let mut is_running = true;

    let mut old_time: f64 = 0.0;
    let mut accumulator: f64 = 0.0; 

    let timer = Instant::now();

    while is_running
    {
        let frame_time = timer.elapsed().as_secs_f64() - old_time;
        old_time = timer.elapsed().as_secs_f64();
        accumulator += frame_time;

        game.process_input();

        while accumulator > 1.0 / 60.0 
        {
            game.update();
            accumulator -= 1.0 / 60.0;
        }
        game.draw(&graphics_device);

        is_running = sdl2_system.keep_window_open();
    }

    Ok(())
}