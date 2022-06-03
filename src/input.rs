pub mod mouse;
pub mod gamepad;
pub mod keyboard;

use mouse::Mouse;
use gamepad::Gamepad;
use keyboard::Keyboard;
use sdl2::{event::Event, GameControllerSubsystem};

pub struct Input
{
    pub mouse: Mouse,
    pub gamepad: Gamepad,
    pub keyboard: Keyboard,
}

impl Input 
{
    pub fn new(mouse: Mouse, gamepad: Gamepad, keyboard: Keyboard ) -> Self {
        Self { mouse, gamepad, keyboard }
    }
    
    pub fn poll(&mut self, game_controller_subsystem: &GameControllerSubsystem, event: &Event) 
    {
        self.mouse.poll(event);
        self.gamepad.poll(game_controller_subsystem, event);
        self.keyboard.poll(event);     
    }

    pub fn clear(&mut self) 
    {
        self.mouse.clear();
        self.gamepad.clear();
        self.keyboard.clear();
    }
}