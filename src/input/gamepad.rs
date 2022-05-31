use crate::math::Vector2;
use std::collections::HashMap;
pub use sdl2::controller::Button as GamePadButton;
use sdl2::{event::Event, controller::GameController, controller::Axis};


const CONTROLLER_RANGE: f32 = 32767.0;

pub struct Gamepad
{
    controller_count: u32,
    left_trigger: HashMap<u32, f32>,
    right_trigger: HashMap<u32, f32>,
    left_stick: HashMap<u32, Vector2<f32>>,
    right_stick: HashMap<u32, Vector2<f32>>,
    controllers: HashMap<u32, GameController>,
    buttons_down: HashMap<u32, GamePadButton>,
    buttons_pressed: HashMap<u32, GamePadButton>,
    buttons_released: HashMap<u32, GamePadButton>,
}

impl Gamepad 
{
    pub fn new() -> Self
    {
        Self 
        {
            controller_count: 0,
            left_stick: HashMap::new(),
            right_stick: HashMap::new(),
            left_trigger: HashMap::new(),
            right_trigger: HashMap::new(),
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
            Some(Event::ControllerAxisMotion { which, axis: Axis::TriggerLeft, value: val, .. }) =>
            {
                self.left_trigger.insert(*which, (*val as f32) / CONTROLLER_RANGE);
            } 
            Some(Event::ControllerAxisMotion { which, axis: Axis::TriggerRight, value: val, .. }) =>
            {
                self.right_trigger.insert(*which, (*val as f32) / CONTROLLER_RANGE);
            },
            Some(Event::ControllerAxisMotion { which, axis, value: val, .. }) =>
            {
            },
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

    pub fn left_trigger(&self, controller_index: u32) -> f32
    {
        match self.left_trigger.get(&controller_index) {
            Some(trigger_value ) => { return *trigger_value }
            None => { return 0.0 }
        }
    }

    pub fn right_trigger(&self, controller_index: u32) -> f32
    {
        match self.right_trigger.get(&controller_index) {
            Some(trigger_value ) => { return *trigger_value }
            None => { return 0.0 }
        }
    }

    pub fn set_rumble(&mut self, force_left: f32, force_right: f32, time: u32, controller_index: u32)
    {
        let force_left_value = (force_left * CONTROLLER_RANGE) as u16;
        let force_right_value = (force_right * CONTROLLER_RANGE) as u16;

        match self.controllers.get_mut(&controller_index)
        {
            Some(controller) => { controller.set_rumble(force_left_value, force_right_value, time).unwrap(); },
            None => todo!(),
        }
    }

    // THIS IS AWFUL FIX IT
    pub fn left_stick(&mut self, controller_index: u32) -> Vector2<i32>
    {
        match self.controllers.get_mut(&controller_index)
        {
            Some(controller) => 
            {  
                return Vector2 
                { 
                    x: ((controller.axis(Axis::LeftX) as i32) / CONTROLLER_RANGE as i32), 
                    y: ((controller.axis(Axis::LeftY) as i32) / CONTROLLER_RANGE as i32)
                }
            },
            None => todo!(),
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