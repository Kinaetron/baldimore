use crate::game::Game;
use system_sdl::SDLSystem;
use spin_sleep::LoopHelper;
use crate::platform::system_sdl;
use crate::input::keyboard::Keyboard;
use crate::graphics::spritebatch::SpriteBatch;
use crate::platform::graphics_interface::GraphicsInterface;

pub struct Window
{
    pub sdl2_system: SDLSystem,
    pub graphics_interface: GraphicsInterface,
}

impl Window 
{
    pub fn new(window_title: &str, width: u32, height: u32) -> Self
    {
        let sdl2_system = SDLSystem::new(window_title, width, height);
        let graphics_interface = GraphicsInterface::new(&sdl2_system).unwrap();

        Self { sdl2_system, graphics_interface }
    }
}


pub fn run(mut window: Window, mut game: impl Game) -> Result<(), String>
{

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(60.5);


    let mut sprite_batch = SpriteBatch::new(window.graphics_interface);

    let mut keyboard = Keyboard::new();

    while keyboard.is_running
    {
        loop_helper.loop_start();

        keyboard.clear();
        keyboard.poll(&window.sdl2_system.next_event());

        game.process_input(&keyboard);
        game.update();
        game.draw(&mut sprite_batch);

        loop_helper.loop_sleep();
    }

    Ok(())
}