use crate::game::Game;
use system_sdl::SDLSystem;
use crate::platform::system_sdl;
use crate::platform::graphics_device::GraphicsDevice;

use std::time::{Instant, Duration};

pub fn run(window_title: &str, width: u32, height: u32, game: impl Game) -> Result<(), String>
{
    let sdl2_system = SDLSystem::new(window_title, width, height);
    let graphics_device = GraphicsDevice::new(&sdl2_system).unwrap();

    let mut is_running = true;

    let zero_duration = Duration::new(0, 0);

    let timer = Instant::now();
    let mut old_time = zero_duration;
    let mut accumulator = zero_duration; 
    let frame_time_cap = Duration::new(0 , 16666666);

    while is_running
    {
        let frame_time = timer.elapsed() - old_time;
        old_time = timer.elapsed();
        accumulator += frame_time;

        game.process_input();

        while accumulator > frame_time_cap 
        {
            game.update();
            accumulator = zero_duration;
        }
        game.draw(&graphics_device);

        is_running = sdl2_system.keep_window_open();
    }

    Ok(())
}