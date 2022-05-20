
use sdl2::event::Event;

pub struct SDLSystem
{
    sdl_context: sdl2::Sdl,
    pub window: sdl2::video::Window,
}

impl SDLSystem  
{
    pub fn new(window_title: &str, width: u32, height: u32) -> Self
    {
        let sdl_context = sdl2::init().unwrap();
        let time = sdl_context.timer().unwrap();

        let ticks = time.ticks();

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        Self {sdl_context , window }
    }

    pub fn keep_window_open(&self) -> bool 
    {
        let mut events = self.sdl_context.event_pump().unwrap();

        for event in events.poll_iter()
        {
            if let Event::Quit { .. } = event {
                return false;
            };
        }

        return true;
    }
}