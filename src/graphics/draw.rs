use imageproc::drawing::*;
use imageproc::rect::Rect;
use image::{Rgba, RgbaImage};
use std::{sync::Arc, collections::HashMap};
use crate::{graphics::colour::Colour, rectangle::Rectangle};
use cgmath::{Rad, Vector2, Vector3, Vector4, Matrix4, SquareMatrix};
use crate::{ platform::graphics_interface::{Vertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    batch_began: bool,
    texture_index: u32,
    camera_matrix: Matrix4<f32>,
    texture_vec: Vec<Arc<Texture>>,
    texture_hashmap: HashMap<u64, u32>,
    pub graphics_interface: GraphicsInterface,
    batch_information_hashmap: HashMap<u32, DrawInformation>
}

pub struct DrawInformation
{
    pub indices: Vec<u16>,
    pub vertices: Vec<Vertex>,
}

impl DrawInformation
{
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { indices, vertices }
    }
}

impl Draw
{
    pub fn new(graphics_interface: GraphicsInterface) -> Self 
    {
        let camera_matrix = Matrix4::identity();
        let texture_hashmap: HashMap<u64, u32> = HashMap::new();
        let texture_vec: Vec<Arc<Texture>> = Vec::with_capacity(16);
        let batch_information_hashmap: HashMap<u32, DrawInformation> = HashMap::new();

        Self { batch_began: false, texture_index: 0, texture_vec, texture_hashmap, graphics_interface, batch_information_hashmap, camera_matrix }
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

        let vertex_1 = Vertex { position: [ vertex_position_1.x, vertex_position_2.y], tex_coords: [left_tex_coord,    bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom left
        let vertex_2 = Vertex { position: [ vertex_position_2.x, vertex_position_2.y], tex_coords: [left_tex_coord,       top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top left
        let vertex_3 = Vertex { position: [ vertex_position_3.x, vertex_position_3.y], tex_coords: [right_tex_coord,      top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top right
        let vertex_4 = Vertex { position: [ vertex_position_4.x, vertex_position_4.y], tex_coords: [right_tex_coord,   bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] };  // bottom right        

        let index = self.texture_hashmap.get(&texture.id);

        if index == None
        {
            if self.texture_index > 15
            {
                self.graphics_interface.batch_render(&self.batch_information_vec);        
                self.batch_information_hashmap.clear();
            }

            self.texture_index += 1;
            self.texture_vec.push(texture);
            self.texture_hashmap.insert(texture.id, self.texture_index);

            let mut vertices: Vec<Vertex>  = Vec::new();
            vertices.push(vertex_1);
            vertices.push(vertex_2);
            vertices.push(vertex_3);
            vertices.push(vertex_4);

            let batch_information = DrawInformation::new(vertices, vec![0, 1, 3, 1 ,2 ,3 ]);
            self.batch_information_hashmap.insert(self.texture_index, batch_information);
        }
        else
        {
           match self.batch_information_hashmap.get(index.unwrap())
           {
                Some(draw_information) => 
                { 
                    draw_information.vertices.push(vertex_1);
                    draw_information.vertices.push(vertex_2);
                    draw_information.vertices.push(vertex_3);
                    draw_information.vertices.push(vertex_4);

                    let indice_index = draw_information.indices.len() - 7;

                    draw_information.indices.push(draw_information.indices[indice_index] + 4);
                    draw_information.indices.push(draw_information.indices[indice_index + 1] + 4);
                    draw_information.indices.push(draw_information.indices[indice_index + 2] + 4);
                    draw_information.indices.push(draw_information.indices[indice_index + 3] + 4);
                }
                None => { panic!("Draw doesn't have vertex information for texture {}", texture.id) }
           }
        }
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

        
        self.batch_information_hashmap.clear();
        self.batch_began = false;
    }
}