use crate::game::Game;
use crate::graphics::spritebatch::SpriteBatch;
use system_sdl::SDLSystem;
use spin_sleep::LoopHelper;
use crate::platform::system_sdl;
use crate::platform::graphics_device::GraphicsDevice;

pub fn run(window_title: &str, width: u32, height: u32, game: impl Game) -> Result<(), String>
{

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(60);

    let sdl2_system = SDLSystem::new(window_title, width, height);


    let graphics_device = GraphicsDevice::new(&sdl2_system).unwrap();
    let mut sprite_batch = SpriteBatch::new(graphics_device);

    let mut is_running = true;

    while is_running
    {
        loop_helper.loop_start();

        game.process_input();
        game.update();
        game.draw(&mut sprite_batch);
        is_running = sdl2_system.keep_window_open();

        loop_helper.loop_sleep();
    }

    Ok(())
}