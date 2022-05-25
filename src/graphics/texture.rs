use std::fs;
use std::path::Path;
use image::{GenericImageView, EncodableLayout};

#[derive(Clone)]
pub struct Texture
{
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Texture 
{
    pub fn new(file_path: &str) -> Self
    {
        let data = fs::read(Path::new(file_path)).unwrap();

        let image = image::load_from_memory(&data).unwrap();
        let dimesions = image.dimensions();

        let rgba = image.as_rgba8().unwrap();
        let rgba_byes = rgba.as_bytes();
        let rgba_vec = rgba_byes.to_vec();

        Self { rgba: rgba_vec, width: dimesions.0, height: dimesions.1 }
    }
}