use sdl2::{event::Event, controller::GameController};
use std::collections::HashMap;
pub use sdl2::controller::Button as GamePadButton;

pub struct Gamepad
{
    controller_count: u32,
    buttons_down: HashMap<u32, GamePadButton>,
    buttons_pressed: HashMap<u32, GamePadButton>,
    buttons_released: HashMap<u32, GamePadButton>,
    controllers: HashMap<u32, GameController>
}

impl Gamepad 
{
    pub fn new() -> Self
    {
        Self 
        {  
            
            controller_count: 0,
            buttons_down: HashMap::new(), 
            buttons_pressed: HashMap::new(), 
            buttons_released: HashMap::new(),
            controllers: HashMap::new()
        }
    }

    pub fn poll(&mut self, game_controller_subsystem: &sdl2::GameControllerSubsystem, event: &Option<Event>)
    {        
        self.open_controllers(game_controller_subsystem);

        match event
        {
            Some(Event::ControllerButtonDown { which, button, .. }) =>
            {
                let was_up = self.buttons_down.insert(*which,*button);
                
                match was_up 
                {
                    Some(_button) => {},
                    None => { self.buttons_pressed.insert(*which,*button); }
                }
            }
            Some(Event::ControllerButtonUp { which, button, .. }) =>
            {
                let was_down = self.buttons_down.remove(which);

                match was_down
                {
                    Some(_button) => {},
                    None => { self.buttons_released.insert(*which, *button); }
                }
            }
            _ => {}
        }
    }

    pub fn clear(&mut self)
    {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
    }

    pub fn is_down(&self, button: GamePadButton, controller_index: u32) -> bool 
    {
          match self.buttons_down.get(&controller_index) {
              Some(match_button) => { return match_button == &button }
              None => { return false }
          }
    }

    pub fn is_up(&self, button: GamePadButton, controller_index: u32) -> bool 
    {
        match self.buttons_down.get(&controller_index) {
            Some(match_button) => { return match_button != &button }
            None => { return false }
        }
    }

    pub fn is_pressed(&self, button: GamePadButton, controller_index: u32) -> bool 
    {
        match self.buttons_pressed.get(&controller_index) {
            Some(match_button) => { return match_button == &button }
            None => { return false }
        }
    }

    pub fn is_released(&self, button: GamePadButton, controller_index: u32) -> bool 
    {        
        match self.buttons_released.get(&controller_index) {
            Some(match_button) => { return match_button == &button }
            None => { return false }
        }
    }

    fn open_controllers(&mut self, game_controller_subsystem: &sdl2::GameControllerSubsystem)
    {
      let available = game_controller_subsystem.num_joysticks().unwrap();

      for id in self.controller_count .. available 
        {
            let game_controller = match game_controller_subsystem.open(id)
            {
                Ok(controller) =>  { self.controller_count += 1; Some(controller)  },
                Err(..) => { None }
            };

            self.controllers.insert(id, game_controller.unwrap());
        }
    }
}