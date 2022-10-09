use fontdue::Font;
use image::RgbaImage;
use crate::shapes::circle::Circle;
use std::{sync::Arc, collections::HashMap, f32::consts::PI};
use crate::{graphics::colour::Colour, shapes::rectangle::Rectangle};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use crate::{math::Rad, math::Vector2, math::Vector3, math::Vector4, math::Matrix4, math::SquareMatrix};
use crate::{platform::graphics_interface::{TextureVertex, GlythVertex, ShapeVertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    glyth_draw_count: u16,
    texture_draw_count: u16,
    rectangle_draw_count: u16,
    circle_draw_count: u16,
    batch_began: bool,
    texture_index: u32,
    texture_indices: Vec<u16>,
    texture_vertices: Vec<TextureVertex>,
    glyth_index: u32,
    glyth_indices: Vec<u16>,
    glyth_vertices: Vec<GlythVertex>,
    rectangle_indices: Vec<u16>,
    rectangle_vertices: Vec<ShapeVertex>,
    circle_indices: Vec<u16>,
    circle_vertices: Vec<ShapeVertex>,
    dummy_glyth: Arc<Texture>,
    dummy_texture: Arc<Texture>,
    camera_matrix: Matrix4<f32>,
    texture_vec: Vec<Arc<Texture>>,
    texture_hashmap: HashMap<u64, u32>,
    glyth_vec: Vec<Arc<Texture>>,
    glyph_hashmap: HashMap<u64, u32>,
    pub graphics_interface: GraphicsInterface,
}

impl Draw
{
    pub fn new(graphics_interface: GraphicsInterface) -> Self 
    {
        let texture_indices: Vec<u16> = Vec::new();
        let texture_vertices: Vec<TextureVertex> = Vec::new();
        let glyth_indices: Vec<u16> = Vec::new();
        let glyth_vertices: Vec<GlythVertex> = Vec::new();
        let rectangle_indices: Vec<u16> = Vec::new();
        let rectangle_vertices: Vec<ShapeVertex> = Vec::new();
        let circle_indices: Vec<u16> = Vec::new();
        let circle_vertices: Vec<ShapeVertex> = Vec::new();
        let camera_matrix = Matrix4::identity();
        let texture_hashmap: HashMap<u64, u32> = HashMap::new();
        let glyph_hashmap: HashMap<u64, u32> = HashMap::new();
        let texture_vec: Vec<Arc<Texture>> = Vec::with_capacity(16);
        let glyth_vec: Vec<Arc<Texture>> = Vec::with_capacity(16);

        let image_buffer = RgbaImage::new(1, 1);
        let dummy_texture =Arc::new(Texture::new_from_buffer(&graphics_interface, image_buffer, Vector2 { x: 1, y: 1 }));

        let mut bitmap: Vec<u8> = Vec::new();
        bitmap.push(1);

        let diemensions = Vector2::new(1, 1);

        let dummy_glyth = Arc::new(Texture::new_glpyh(&graphics_interface, &bitmap, &diemensions));

        Self
        {
            glyth_vec,
            dummy_glyth,
            glyth_index: 0,
            glyth_indices,
            glyth_vertices,
            glyth_draw_count: 0,
            glyph_hashmap,
            texture_draw_count: 0,
            rectangle_draw_count: 0, 
            circle_draw_count: 0,
            texture_indices, 
            batch_began: false,
            dummy_texture, 
            texture_index: 0, 
            texture_vec, 
            texture_hashmap, 
            graphics_interface, 
            texture_vertices, 
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

    pub fn sprite(&mut self, texture: Arc<Texture>, position: &Vector2<f32>, draw_area: &Rectangle, size: Vector2<f32>, rotation: f32,  colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call sprite without calling begin first");
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

        let left_tex_coord = draw_area.left() / (texture.width as f32);
        let right_tex_coord = draw_area.right() / (texture.width as f32);
        let top_tex_coord = 1.0 - draw_area.top() / (texture.height as f32);
        let bottom_tex_coord = 1.0 - draw_area.bottom() / (texture.height as f32);

        let mut vertex_1 = TextureVertex { index: self.texture_index, position: [ vertex_position_1.x, vertex_position_1.y], tex_coords: [left_tex_coord,    bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom left
        let mut vertex_2 = TextureVertex { index: self.texture_index, position: [ vertex_position_2.x, vertex_position_2.y], tex_coords: [left_tex_coord,       top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top left
        let mut vertex_3 = TextureVertex { index: self.texture_index, position: [ vertex_position_3.x, vertex_position_3.y], tex_coords: [right_tex_coord,      top_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // top right
        let mut vertex_4 = TextureVertex { index: self.texture_index, position: [ vertex_position_4.x, vertex_position_4.y], tex_coords: [right_tex_coord,   bottom_tex_coord], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }; // bottom right        

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

        self.texture_vertices.push(vertex_1);
        self.texture_vertices.push(vertex_2);
        self.texture_vertices.push(vertex_3);
        self.texture_vertices.push(vertex_4);

        let index_offset = 4 * self.texture_draw_count;

        self.texture_indices.push(0 + index_offset);
        self.texture_indices.push(1 + index_offset);
        self.texture_indices.push(3 + index_offset);
        self.texture_indices.push(1 + index_offset);
        self.texture_indices.push(2 + index_offset);
        self.texture_indices.push(3 + index_offset);

        self.texture_draw_count += 1;

    }

    pub fn text(&mut self, text: &str, position: &Vector2<f32>, text_size: f32, font: &Vec<u8>,  colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call text without calling begin first");
        }

        let colour = colour.converted_to_color();

        let fonts = &[Font::from_bytes(font.as_slice(), fontdue::FontSettings::default()).unwrap()];
        let mut layout: Layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.append(fonts, &TextStyle::new(text, text_size, 0));

        for i in 0..text.len()
        {
            let character = text.chars().nth(i).unwrap();
            
            if character == ' ' {
                continue;
            }

            let glyth_details = layout.glyphs()[i];
            let (metrics, bitmap) = fonts[0].rasterize(character, text_size);
            let diemensions = Vector2::new(metrics.width as u32, metrics.height as u32);

            let texture = Texture::new_glpyh(&self.graphics_interface, &bitmap, &diemensions);

            let origin_x = diemensions.x as f32 / 2.0;
            let origin_y = diemensions.y as f32 / 2.0;

            let glyth_displacement_x = glyth_details.x + (glyth_details.width / 2) as f32;
            let glyth_displacement_y = glyth_details.y as i32 / 2;

            let model_matrix = Matrix4::from_translation(Vector3 { x: position.x + glyth_displacement_x, y: position.y + glyth_displacement_y as f32,  z: 0.0 });
            let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

            let vertex_position_1 =  final_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
            let vertex_position_2 =  final_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
            let vertex_position_3 =  final_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
            let vertex_position_4 =  final_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

            let mut vertex_1 = GlythVertex { index: self.glyth_index, position: [ vertex_position_1.x, vertex_position_1.y], tex_coords: [0.0, 0.0], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // bottom left
            let mut vertex_2 = GlythVertex { index: self.glyth_index, position: [ vertex_position_2.x, vertex_position_2.y], tex_coords: [0.0, 1.0], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // top left
            let mut vertex_3 = GlythVertex { index: self.glyth_index, position: [ vertex_position_3.x, vertex_position_3.y], tex_coords: [1.0, 1.0], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // top right
            let mut vertex_4 = GlythVertex { index: self.glyth_index, position: [ vertex_position_4.x, vertex_position_4.y], tex_coords: [1.0, 0.0], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // bottom right   

            match self.glyph_hashmap.get(&texture.id)
            {
                Some(index) => 
                {
                   let index_value = *index;

                   vertex_1.index = index_value;
                   vertex_2.index = index_value;
                   vertex_3.index = index_value;
                   vertex_4.index = index_value;
                }
                None =>
                {
                    self.glyph_hashmap.insert(texture.id, self.glyth_index);
                    self.glyth_vec.push(Arc::new(texture));
                    self.glyth_index += 1;
                }
            }
    
            self.glyth_vertices.push(vertex_1);
            self.glyth_vertices.push(vertex_2);
            self.glyth_vertices.push(vertex_3);
            self.glyth_vertices.push(vertex_4);
    
            let index_offset = 4 * self.glyth_draw_count;
    
            self.glyth_indices.push(0 + index_offset);
            self.glyth_indices.push(1 + index_offset);
            self.glyth_indices.push(3 + index_offset);
            self.glyth_indices.push(1 + index_offset);
            self.glyth_indices.push(2 + index_offset);
            self.glyth_indices.push(3 + index_offset);
    
            self.glyth_draw_count += 1;
        }
    }

    pub fn rectangle(&mut self, rectangle: &Rectangle, colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call rectangle without calling begin first");
        }

        let colour = colour.converted_to_color();

        let origin_x = rectangle.width as f32 / 2.0;
        let origin_y = rectangle.height as f32 / 2.0;

        let model_matrix = Matrix4::from_translation(Vector3 { x: rectangle.centre().x, y: rectangle.centre().y,  z: 0.0 });

        let final_matrix = self.graphics_interface.world_matrix * self.camera_matrix  * model_matrix;

        let vertex_position_1 =  final_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_2 =  final_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_3 =  final_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_position_4 =  final_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

        let vertex_1 = ShapeVertex { position: [ vertex_position_1.x, vertex_position_1.y], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // bottom left
        let vertex_2 = ShapeVertex { position: [ vertex_position_2.x, vertex_position_2.y], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // top left
        let vertex_3 = ShapeVertex { position: [ vertex_position_3.x, vertex_position_3.y], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // top right
        let vertex_4 = ShapeVertex { position: [ vertex_position_4.x, vertex_position_4.y], color: [colour.r as f32, colour.g as f32, colour.b as f32, colour.a as f32] }; // bottom right
        
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
            panic!("You can't call circle without calling begin first");
        }

        let color = colour.converted_to_color();

        let model_matrix = Matrix4::from_translation(Vector3 { x: circle.position.x, y: circle.position.y,  z: 0.0 });
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
            panic!("You can't call end without calling begin first");
        }

        let mut x = 0;
        let texture_count = 16 - self.texture_index;

        while x < texture_count
        {
            self.texture_vec.push(self.dummy_texture.clone());
            x += 1;
        }

        let mut y = 0;
        let glyth_count = 16 - self.glyth_index;

        while y < glyth_count
        {
            self.glyth_vec.push(self.dummy_glyth.clone());
            y += 1;
        }

        self.graphics_interface.batch_render(&self.texture_vec, &self.texture_vertices, &self.texture_indices,
                                                      &self.glyth_vec, &self.glyth_vertices, &self.glyth_indices, 
                                                       &self.rectangle_vertices, &self.rectangle_indices,
                                                       &self.circle_vertices, &self.circle_indices);
        self.flush();
    }

    fn flush(& mut self)
    {
        self.glyth_draw_count = 0;
        self.circle_draw_count = 0;
        self.texture_draw_count = 0;
        self.rectangle_draw_count = 0;
        self.glyth_vertices.clear();
        self.glyth_indices.clear();
        self.glyth_index = 0;
        self.glyth_vec.clear();
        self.glyph_hashmap.clear();
        self.circle_vertices.clear();
        self.circle_indices.clear();
        self.rectangle_vertices.clear();
        self.rectangle_indices.clear();
        self.texture_vertices.clear();
        self.texture_indices.clear();
        self.texture_index = 0;
        self.texture_vec.clear();
        self.batch_began = false;
        self.texture_hashmap.clear();
    }
}