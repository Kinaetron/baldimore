use sdl2::event::Event;
use std::collections::HashSet;

pub use sdl2::keyboard::Scancode as Key;

pub struct Keyboard
{
    pub is_running: bool,
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
}

impl Keyboard 
{
    pub fn new() -> Self
    {
        Self 
        { 
            is_running: true,
            keys_down: HashSet::new(), 
            keys_pressed: HashSet::new(), 
            keys_released: HashSet::new() 
        }
    }

    pub fn poll(&mut self, event: &Event)
    {
        match event
        {
            Event::KeyDown { scancode: Some(scancode), repeat: false, .. } =>
            {
                let was_up = self.keys_down.insert(*scancode);
                                
                if was_up {
                    self.keys_pressed.insert(*scancode);
                }
            }
            Event::KeyUp { scancode: Some(scancode), repeat: false, ..  } =>
            {
                let was_down = self.keys_down.remove(scancode);

                if was_down {
                    self.keys_released.insert(*scancode);
                }
            }
            Event::Quit { .. } => {
                self.is_running = false;
            }

            _ => {}
        }
    }

    pub fn clear(&mut self)
    {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn is_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_up(&self, key: Key) -> bool {
        !self.keys_down.contains(&key)
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }
}