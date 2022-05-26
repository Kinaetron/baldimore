use std::fs;
use std::path::Path;
use image::GenericImageView;

#[derive(Clone)]
pub struct Texture
{
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture 
{
    pub fn new(file_path: &str) -> Self
    {
        let data = fs::read(Path::new(file_path)).unwrap();

        let image = image::load_from_memory(&data).unwrap();
        let dimesions = image.dimensions();

        Self { data, width: dimesions.0, height: dimesions.1 }
    }
}