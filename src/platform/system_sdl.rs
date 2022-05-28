
use sdl2::event::Event;

pub struct SDLSystem
{
    sdl_context: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump
}

impl SDLSystem  
{
    pub fn new(window_title: &str, width: u32, height: u32) -> Self
    {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        Self {sdl_context , window, event_pump }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}