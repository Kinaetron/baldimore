
use sdl2::event::Event;

pub struct SDLSystem
{
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump,
    pub game_controller_subsystem: sdl2::GameControllerSubsystem
}

impl SDLSystem  
{
    pub fn new(window_title: &str, width: u32, height: u32) -> Self
    {
        let sdl_context = sdl2::init().unwrap();
        sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");

        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let game_controller_subsystem = sdl_context.game_controller().unwrap();

        let window = video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        Self { window, event_pump, game_controller_subsystem }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}