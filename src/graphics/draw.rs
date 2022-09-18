use image::RgbaImage;
use std::{sync::Arc, collections::HashMap, f32::consts::PI};
use crate::shapes::circle::Circle;
use crate::{graphics::colour::Colour, shapes::rectangle::Rectangle};
use crate::{math::Rad, math::Vector2, math::Vector3, math::Vector4, math::Matrix4, math::SquareMatrix};
use crate::{platform::graphics_interface::{SpriteVertex, ShapeVertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    sprite_draw_count: u16,
    rectangle_draw_count: u16,
    circle_draw_count: u16,
    batch_began: bool,
    texture_index: u32,
    sprite_indices: Vec<u16>,
    sprite_vertices: Vec<SpriteVertex>,
    rectangle_indices: Vec<u16>,
    rectangle_vertices: Vec<ShapeVertex>,
    circle_indices: Vec<u16>,
    circle_vertices: Vec<ShapeVertex>,
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

        let sprite_indices: Vec<u16> = Vec::new();
        let sprite_vertices: Vec<SpriteVertex> = Vec::new();
        let rectangle_indices: Vec<u16> = Vec::new();
        let rectangle_vertices: Vec<ShapeVertex> = Vec::new();
        let circle_indices: Vec<u16> = Vec::new();
        let circle_vertices: Vec<ShapeVertex> = Vec::new();
        let camera_matrix = Matrix4::identity();
        let texture_hashmap: HashMap<u64, u32> = HashMap::new();
        let texture_vec: Vec<Arc<Texture>> = Vec::with_capacity(16);

        let image_buffer = RgbaImage::new(1, 1);
        let dummy_texture =Arc::new(Texture::new_from_buffer(&graphics_interface, image_buffer, Vector2 { x: 1, y: 1 }));

        Self 
        { 
            sprite_draw_count: 0,
            rectangle_draw_count: 0, 
            circle_draw_count: 0,
            sprite_indices, 
            batch_began: false,
            dummy_texture, 
            texture_index: 0, 
            texture_vec, 
            texture_hashmap, 
            graphics_interface, 
            sprite_vertices, 
            camera_matrix, 
            rectangle_indices,
            rectangle_vertices,
            circle_indices,
            circle_vertices
        }
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

        let mut vertex_1 = SpriteVertex { index: self.texture_index, position: [ vertex_position_1.x, vertex_position_1.y], tex_coords: [left_tex_coord,    bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom left
        let mut vertex_2 = SpriteVertex { index: self.texture_index, position: [ vertex_position_2.x, vertex_position_2.y], tex_coords: [left_tex_coord,       top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top left
        let mut vertex_3 = SpriteVertex { index: self.texture_index, position: [ vertex_position_3.x, vertex_position_3.y], tex_coords: [right_tex_coord,      top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top right
        let mut vertex_4 = SpriteVertex { index: self.texture_index, position: [ vertex_position_4.x, vertex_position_4.y], tex_coords: [right_tex_coord,   bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom right        

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

        self.sprite_vertices.push(vertex_1);
        self.sprite_vertices.push(vertex_2);
        self.sprite_vertices.push(vertex_3);
        self.sprite_vertices.push(vertex_4);

        let index_offset = 4 * self.sprite_draw_count;

        self.sprite_indices.push(0 + index_offset);
        self.sprite_indices.push(1 + index_offset);
        self.sprite_indices.push(3 + index_offset);
        self.sprite_indices.push(1 + index_offset);
        self.sprite_indices.push(2 + index_offset);
        self.sprite_indices.push(3 + index_offset);

        self.sprite_draw_count += 1;

    }

    pub fn rectangle(&mut self, rectangle: &Rectangle, colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        let color = colour.converted_to_color();

        let origin_x = rectangle.width as f32 / 2.0;
        let origin_y = rectangle.height as f32 / 2.0;

        let model_matrix = Matrix4::from_translation(Vector3 { x: rectangle.centre.x, y: rectangle.centre.y,  z: 0.0 });

        let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

        let vertex_position_1 =  final_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_2 =  final_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_3 =  final_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_4 =  final_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

        let vertex_1 = ShapeVertex { position: [ vertex_position_1.x, vertex_position_1.y], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom left
        let vertex_2 = ShapeVertex { position: [ vertex_position_2.x, vertex_position_2.y], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top left
        let vertex_3 = ShapeVertex { position: [ vertex_position_3.x, vertex_position_3.y], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top right
        let vertex_4 = ShapeVertex { position: [ vertex_position_4.x, vertex_position_4.y], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom right
        
        self.rectangle_vertices.push(vertex_1);
        self.rectangle_vertices.push(vertex_2);
        self.rectangle_vertices.push(vertex_3);
        self.rectangle_vertices.push(vertex_4);

        let index_offset = 4 * self.rectangle_draw_count;

        self.rectangle_indices.push(0 + index_offset);
        self.rectangle_indices.push(1 + index_offset);
        self.rectangle_indices.push(1 + index_offset);
        self.rectangle_indices.push(2 + index_offset);
        self.rectangle_indices.push(2 + index_offset);
        self.rectangle_indices.push(3 + index_offset);
        self.rectangle_indices.push(3 + index_offset);
        self.rectangle_indices.push(0 + index_offset);

        self.rectangle_draw_count += 1;
    }

    pub fn circle(&mut self, circle: &Circle, colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }
        let color = colour.converted_to_color();

        let model_matrix = Matrix4::from_translation(Vector3 { x: circle.centre.x, y: circle.centre.y,  z: 0.0 });
        let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

        let mut vertex_index:f32 = 0.0;

        while vertex_index < 33.0
        {
            let vertex_position = final_matrix * Vector4 { x: ((vertex_index * (PI / 16.0))).cos() * circle.radius, y: ((vertex_index * (PI / 16.0))).sin() * circle.radius, z: 0.0, w: 1.0 };
            let vertex = ShapeVertex { position: [ vertex_position.x, vertex_position.y], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] };

            self.circle_vertices.push(vertex);
            vertex_index += 1.0;
        }

        let mut index = 0;
        let index_offset = 33 * self.circle_draw_count;

        while index < 32
        {
            self.circle_indices.push(index + index_offset);
            self.circle_indices.push((index + 1) + index_offset);

            index += 1;
        }

        self.circle_indices.push(32 + index_offset);
        self.circle_indices.push(0 + index_offset);

        self.circle_draw_count += 1;
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

        self.graphics_interface.batch_render(&self.texture_vec, &self.sprite_vertices, &self.sprite_indices, 
                                                       &self.rectangle_vertices, &self.rectangle_indices,
                                                       &self.circle_vertices, &self.circle_indices);
        self.flush();
    }

    fn flush(& mut self)
    {
        self.circle_draw_count = 0;
        self.sprite_draw_count = 0;
        self.rectangle_draw_count = 0;
        self.circle_vertices.clear();
        self.circle_indices.clear();
        self.rectangle_vertices.clear();
        self.rectangle_indices.clear();
        self.sprite_vertices.clear();
        self.sprite_indices.clear();
        self.texture_index = 0;
        self.texture_vec.clear();
        self.batch_began = false;
        self.texture_hashmap.clear();
    }
}