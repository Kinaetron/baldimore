use std::fs::File;
use crate::math::*;
use image::GenericImageView;
use crate::platform::graphics_device::GraphicsDevice;

pub struct Render {
    graphics_device: GraphicsDevice
}

impl Render
{
    pub fn new(graphics_device: GraphicsDevice) -> Self {
        Self { graphics_device }    
    }

    pub fn Sprite(&self, filepath: &str,  position: Vector2<i32>) 
    {
        let diffuse_bytes = File::open(filepath);
    }
}