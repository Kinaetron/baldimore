use imageproc::drawing::*;
use imageproc::rect::Rect;
use image::{Rgba, RgbaImage};
use std::{sync::Arc, collections::HashMap};
use crate::{graphics::colour::Colour, rectangle::Rectangle};
use cgmath::{Rad, Vector2, Vector3, Vector4, Matrix4, SquareMatrix};
use crate::{platform::graphics_interface::{Vertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    draw_count: u16,
    indices: Vec<u16>,
    batch_began: bool,
    texture_index: u32,
    vertices: Vec<Vertex>,
    dummy_texture: Arc<Texture>,
    camera_matrix: Matrix4<f32>,
    texture_vec: Vec<Arc<Texture>>,
    texture_hashmap: HashMap<u64, u32>,
    pub graphics_interface: GraphicsInterface,
}

impl Draw
{
    pub fn new(graphics_interface: GraphicsInterface) -> Self 
    {

        let indices: Vec<u16> = Vec::new();
        let vertices: Vec<Vertex> = Vec::new();
        let camera_matrix = Matrix4::identity();
        let texture_hashmap: HashMap<u64, u32> = HashMap::new();
        let texture_vec: Vec<Arc<Texture>> = Vec::with_capacity(16);

        let image_buffer = RgbaImage::new(1, 1);
        let dummy_texture =Arc::new(Texture::new_from_buffer(&graphics_interface, image_buffer, Vector2 { x: 1, y: 1 }));

        Self { draw_count: 0, indices, batch_began: false, dummy_texture, texture_index: 0, texture_vec, texture_hashmap, graphics_interface, vertices, camera_matrix }
    }

    pub fn clear(&mut self, colour: Colour) 
    {   let color = colour.converted_to_color();
        self.graphics_interface.clear(color.r, color.g, color.b, color.a)
    }

    pub fn begin(&mut self, camera_matrix: Matrix4<f32>) {

        if self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        self.batch_began = true;
        self.camera_matrix = camera_matrix;
    }

    pub fn sprite(&mut self, texture: Arc<Texture>, position: Vector2<f32>, draw_area: &Rectangle, size: Vector2<f32>, rotation: f32,  colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        let color = colour.converted_to_color();

        let origin_x = size.x as f32 / 2.0;
        let origin_y = size.y as f32 / 2.0;

        let mut model_matrix = Matrix4::from_translation(Vector3 { x: position.x, y: position.y,  z: 0.0 });
        model_matrix = model_matrix * Matrix4::from_angle_z(Rad(rotation));

        let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

        let vertex_position_1 =  final_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_2 =  final_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_3 =  final_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_4 =  final_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

        let left_tex_coord = draw_area.left / (texture.width as f32);
        let right_tex_coord = draw_area.right / (texture.width as f32);
        let top_tex_coord = 1.0 - draw_area.top / (texture.height as f32);
        let bottom_tex_coord = 1.0 - draw_area.bottom / (texture.height as f32);

        let mut vertex_1 = Vertex { index: self.texture_index, position: [ vertex_position_1.x, vertex_position_1.y], tex_coords: [left_tex_coord,    bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom left
        let mut vertex_2 = Vertex { index: self.texture_index, position: [ vertex_position_2.x, vertex_position_2.y], tex_coords: [left_tex_coord,       top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top left
        let mut vertex_3 = Vertex { index: self.texture_index, position: [ vertex_position_3.x, vertex_position_3.y], tex_coords: [right_tex_coord,      top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top right
        let mut vertex_4 = Vertex { index: self.texture_index, position: [ vertex_position_4.x, vertex_position_4.y], tex_coords: [right_tex_coord,   bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom right        

        match self.texture_hashmap.get(&texture.id)
        {
            Some(index) => 
            {
               let index_value = *index;
                
                if index_value > 15 {
                    self.end();
                }

               vertex_1.index = index_value;
               vertex_2.index = index_value;
               vertex_3.index = index_value;
               vertex_4.index = index_value;
            }
            None =>
            { 
                self.texture_hashmap.insert(texture.id, self.texture_index);
                self.texture_vec.push(texture);
                self.texture_index += 1;
            }
        }

        self.vertices.push(vertex_1);
        self.vertices.push(vertex_2);
        self.vertices.push(vertex_3);
        self.vertices.push(vertex_4);

        let index_offset = 4 * self.draw_count;

        self.indices.push(0 + index_offset);
        self.indices.push(1 + index_offset);
        self.indices.push(3 + index_offset);
        self.indices.push(1 + index_offset);
        self.indices.push(2 + index_offset);
        self.indices.push(3 + index_offset);

        self.draw_count += 1;

    }

    pub fn rectangle(&mut self, position: Vector2<f32>, size: Vector2<u32>, colour: Colour)
    {
        let rectangle = Rect::at(0, 0).of_size(size.x, size.y);
        let color = Rgba([colour.red as u8, colour.green as u8, colour.blue as u8, colour.alpha as u8]);

        let image_buffer = RgbaImage::new(size.x, size.y);
        let result = draw_hollow_rect(&image_buffer, rectangle, color);
        
        let new_size = Vector2 { x: size.x as f32 , y: size.y as f32 };
        let draw_area = Rectangle::new(0.0, 0.0, size.x as f32 , size.y as f32);
        let texture = Arc::new(Texture::new_from_buffer(&self.graphics_interface, result, size));

        self.sprite(Arc::clone(&texture), position, &draw_area, new_size, 0.0, colour)
    }

    pub fn circle(&mut self, position: Vector2<f32>, radius: u32, colour: Colour)
    {
        let length = radius * 3;
        let color = Rgba([colour.red as u8, colour.green as u8, colour.blue as u8, colour.alpha as u8]);

        let image_buffer = RgbaImage::new(length, length);
        let result = draw_hollow_circle(&image_buffer, ((length / 2) as i32,  (length / 2) as i32), radius as i32, color);

        let new_size = Vector2 { x: length as f32 , y: length as f32 };
        let draw_area = Rectangle::new(0.0, 0.0, length as f32 , length as f32);
        let texture = Arc::new(Texture::new_from_buffer(&self.graphics_interface, result, Vector2 { x: length, y: length }));

        self.sprite(Arc::clone(&texture), position, &draw_area, new_size, 0.0, colour)
    }

    pub fn end(&mut self)
    {
        if !self.batch_began {
            panic!("You can't call end if without calling begin first");
        }

        let mut x = 0;
        let count = 16 - self.texture_index;

        while x < count
        {
            self.texture_vec.push(self.dummy_texture.clone());
            x += 1;
        }

        self.graphics_interface.batch_render(&self.texture_vec, &self.vertices, &self.indices);
        self.flush();
    }

    fn flush(& mut self)
    {
        self.draw_count = 0;
        self.indices.clear();
        self.vertices.clear();
        self.texture_index = 0;
        self.texture_vec.clear();
        self.batch_began = false;
        self.texture_hashmap.clear();
    }
}