use sdl2::event::Event;
use crate::math::Vector2;
use std::collections::HashSet;

pub use sdl2::mouse::MouseButton as MouseButton;
pub struct Mouse
{
    pub position: Vector2<f32>,
    buttons_down: HashSet<MouseButton>,
    buttons_pressed: HashSet<MouseButton>,
    buttons_released: HashSet<MouseButton>,
}

impl Mouse 
{
    pub fn new() -> Self
    {
        Self 
        { 
            buttons_down: HashSet::new(), 
            buttons_pressed: HashSet::new(), 
            buttons_released: HashSet::new(),
            position: Vector2 { x: 0.0, y:  0.0 }
        }
    }

    pub fn poll(&mut self, event: &Event)
    {
        match event
        {
            Event::MouseButtonDown { mouse_btn, .. } =>
            {
                let was_up = self.buttons_down.insert(*mouse_btn);
                                
                if was_up {
                    self.buttons_pressed.insert(*mouse_btn);
                }
            }
            Event::MouseButtonUp { mouse_btn, .. } =>
            {
                let was_down = self.buttons_down.remove(mouse_btn);

                if was_down {
                    self.buttons_released.insert(*mouse_btn);
                }
            }
            Event::MouseMotion { x, y, .. } => {
                self.position = Vector2::new(*x as f32, *y as f32);
            }

            _ => {}
        }
    }

    pub fn clear(&mut self)
    {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
    }

    pub fn is_down(&self, mouse_button: MouseButton) -> bool {
        self.buttons_down.contains(&mouse_button)
    }

    pub fn is_up(&self, mouse_button: MouseButton) -> bool {
        !self.buttons_down.contains(&mouse_button)
    }

    pub fn is_pressed(&self, mouse_button: MouseButton) -> bool {
        self.buttons_pressed.contains(&mouse_button)
    }

    pub fn is_released(&self, mouse_button: MouseButton) -> bool {
        self.buttons_released.contains(&mouse_button)
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

}