use crate::game::Game;
use std::time::Instant;
use crate::input::Input;
use system_sdl::SDLSystem;
use crate::platform::system_sdl;
use crate::graphics::draw::Draw;
use crate::input::{ gamepad::Gamepad, keyboard::Keyboard, mouse::Mouse };
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
        let graphics_interface = match GraphicsInterface::new(&sdl2_system)
        {
            Ok(graphics_interface) => { graphics_interface },
            Err(e) => {   
                panic!("couldn't create graphics interface for window, error message: {}", e) 
            }
        };
        Self { sdl2_system, graphics_interface }
    }
}


pub fn run(mut window: Window, mut game: impl Game) -> Result<(), String>
{
    let mut sprite_batch = Draw::new(window.graphics_interface);

    let mouse = Mouse::new();
    let gamepad = Gamepad::new();
    let keyboard = Keyboard::new();

    let mut input = Input::new(mouse, gamepad, keyboard);


    let mut accumulator = 0.0;
    let timer = Instant::now();
    let mut previous_time = timer.elapsed().as_secs_f64();

    while input.keyboard.is_running
    {
        let current_time = timer.elapsed().as_secs_f64();
        let elapsed_time = current_time - previous_time;
        previous_time = current_time;
        accumulator += elapsed_time;

        for event in window.sdl2_system.event_pump.poll_iter() {
            input.poll(&window.sdl2_system.game_controller_subsystem, &event);
        }

        game.process_input(&mut input);

        while accumulator >= 1.0 / 60.0
        {
            game.update();
            accumulator -= 1.0 / 60.0;
        }

        game.draw(&mut sprite_batch);
        input.clear();
    }

    Ok(())
}