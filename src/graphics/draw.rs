use std::{sync::Arc};
use image::{Rgba, RgbaImage};
use imageproc::drawing::*;
use imageproc::rect::Rect;
use crate::{graphics::colour::Colour, rectangle::Rectangle};
use cgmath::{Rad, Vector2, Vector3, Vector4, Matrix4, SquareMatrix};
use crate::{ platform::graphics_interface::{Vertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    batch_began: bool,
    camera_matrix: Matrix4<f32>,
    pub graphics_interface: GraphicsInterface,
    batch_information_vec: Vec<DrawInformation>
}


pub struct DrawInformation
{
    pub indices: Vec<u16>,
    pub texture: Arc<Texture>,
    pub vertices: Vec<Vertex>,
}

impl DrawInformation
{
    pub fn new(texture: Arc<Texture>, vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { texture, indices, vertices }
    }
}

impl Draw
{
    pub fn new(graphics_interface: GraphicsInterface) -> Self 
    {
        let camera_matrix = Matrix4::identity();
        let batch_information_vec: Vec<DrawInformation> = Vec::new();

        Self { batch_began: false, graphics_interface, batch_information_vec, camera_matrix }
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

    pub fn sprite(&mut self, texture: Arc<Texture>, position: Vector2<f32>, draw_area: Rectangle, size: Vector2<f32>, rotation: f32,  colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }
        
        let mut vertices: Vec<Vertex>  = Vec::new();
        let indices = vec![0, 1, 3, 1 ,2 ,3 ];

        let color = colour.converted_to_color();

        let origin_x = size.x as f32 / 2.0;
        let origin_y = size.y as f32 / 2.0;

        let mut model_matrix = Matrix4::from_translation(Vector3 { x: position.x, y: position.y,  z: 0.0 });
        model_matrix = model_matrix * Matrix4::from_angle_z(Rad(rotation));

        let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

        let vertex_1 =  final_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
        let vertex_2 =  final_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_3 =  final_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_4 =  final_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

        let left_tex_coord = draw_area.left / (texture.width as f32);
        let right_tex_coord = draw_area.right / (texture.width as f32);
        let top_tex_coord = 1.0 - draw_area.top / (texture.height as f32);
        let bottom_tex_coord = 1.0 - draw_area.bottom / (texture.height as f32);

        vertices.push(Vertex { position: [ vertex_1.x, vertex_1.y], tex_coords: [left_tex_coord,  bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // bottom left
        vertices.push(Vertex { position: [ vertex_2.x, vertex_2.y], tex_coords: [left_tex_coord,     top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // top left
        vertices.push(Vertex { position: [ vertex_3.x, vertex_3.y], tex_coords: [right_tex_coord,    top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // top right
        vertices.push(Vertex { position: [ vertex_4.x, vertex_4.y], tex_coords: [right_tex_coord, bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // bottom right

        let batch_information = DrawInformation::new(texture, vertices, indices);
        self.batch_information_vec.push(batch_information);
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

        self.sprite(Arc::clone(&texture), position, draw_area, new_size, 0.0, colour)
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

        self.sprite(Arc::clone(&texture), position, draw_area, new_size, 0.0, colour)
    }

    pub fn end(&mut self)
    {
        if !self.batch_began {
            panic!("You can't call end if without calling begin first");
        }

        self.graphics_interface.batch_render(&self.batch_information_vec);

        
        self.batch_information_vec.clear();
        self.batch_began = false;
    }
}