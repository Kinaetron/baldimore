use std::sync::Arc;
use crate::graphics::colour::Colour;
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


    pub fn sprite(&mut self, texture: Arc<Texture>, position: Vector2<f32>, rotation: f32,  colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }
        
        let mut vertices: Vec<Vertex>  = Vec::new();
        let indices = vec![0, 1, 3, 1 ,2 ,3 ];

        let color = colour.converted_to_color();

        let origin_x = (texture.width / 2) as f32;
        let origin_y = (texture.height / 2) as f32;

        let mut model_matrix = Matrix4::from_translation(Vector3 { x: position.x, y: position.y,  z: 0.0 });
        model_matrix = model_matrix * Matrix4::from_angle_z(Rad(rotation));

        let finan_matrix = self.camera_matrix * self.graphics_interface.world_matrix  * model_matrix;

        let vertex_1 =  finan_matrix * Vector4 { x: -origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };
        let vertex_2 =  finan_matrix * Vector4 { x: -origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_3 =  finan_matrix * Vector4 { x:  origin_x, y:   origin_y,  z: 0.0, w: 1.0 };
        let vertex_4 =  finan_matrix * Vector4 { x:  origin_x, y:  -origin_y,  z: 0.0, w: 1.0 };

        vertices.push(Vertex { position: [ vertex_1.x, vertex_1.y, vertex_1.z, vertex_1.w ], tex_coords: [0.0, 0.0], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // bottom left
        vertices.push(Vertex { position: [ vertex_2.x, vertex_2.y, vertex_2.z, vertex_2.w ], tex_coords: [0.0, 1.0], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // top left
        vertices.push(Vertex { position: [ vertex_3.x, vertex_3.y, vertex_3.z, vertex_3.w ], tex_coords: [1.0, 1.0], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // top right
        vertices.push(Vertex { position: [ vertex_4.x, vertex_4.y, vertex_4.z, vertex_4.w ], tex_coords: [1.0, 0.0], color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32] }); // bottom right

        let batch_information = DrawInformation::new(texture, vertices, indices);
        self.batch_information_vec.push(batch_information);
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