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
        let sdl_context = match sdl2::init() 
        {
            Ok(sdl_context) => { sdl_context },
            Err(e) => { panic!("couldn't create sdl context in system sdl, error message: {}", e) }
        };

        let event_pump = match sdl_context.event_pump()
        {
            Ok(event_pump) => { event_pump },
            Err(e) => { panic!("couldn't create sdl event pump in system sdl, error message: {}", e) }
        };

        let video_subsystem = match sdl_context.video()
        {
            Ok(video_subsystem) => { video_subsystem },
            Err(e) => { panic!("couldn't create sdl video subsystem in system sdl, error message: {}", e) }
        };


        let game_controller_subsystem = match sdl_context.game_controller()
        {
            Ok(video_subsystem) => { video_subsystem },
            Err(e) => { panic!("couldn't create sdl game controller subsystem in system sdl, error message: {}", e) }
        };

        let window = match video_subsystem
            .window(window_title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            {
                Ok(window) => { window },
                Err(e) => { panic!("couldn't create sdl window in system sdl, error message: {}", e) }
            };

        Self { window, event_pump, game_controller_subsystem }
    }
}