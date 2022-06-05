use std::sync::Arc;
use crate::graphics::colour::Colour;

use crate::{math::Vector2, platform::graphics_interface::{Vertex, GraphicsInterface}, graphics::texture::Texture};

pub struct Draw
{
    batch_began: bool,
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
        let batch_information_vec: Vec<DrawInformation> = Vec::new();

        Self { batch_began: false, graphics_interface, batch_information_vec }
    }

    pub fn begin(&mut self) {

        if self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        self.batch_began = true;
    }


    pub fn draw(&mut self, texture: Arc<Texture>, position: Vector2<f32>, colour: Colour)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        let origin_x = (texture.width / 2) as f32;
        let origin_y = (texture.height / 2) as f32;

        let mut vertices: Vec<Vertex>  = Vec::new();
        let indices = vec![0, 1, 3, 1 ,2 ,3 ];

        let color = colour.converted_to_color();

        vertices.push(Vertex { position: [ position.x - origin_x, position.y - origin_y], tex_coords: [0.0, 0.0], color: [color.r, color.g , color.b, color.a ] }); // bottom left
        vertices.push(Vertex { position: [ position.x - origin_x, position.y + origin_y], tex_coords: [0.0, 1.0], color: [color.r, color.g , color.b, color.a ] }); // top left
        vertices.push(Vertex { position: [ position.x + origin_x, position.y + origin_y], tex_coords: [1.0, 1.0], color: [color.r, color.g , color.b, color.a ] }); // top right
        vertices.push(Vertex { position: [ position.x + origin_x, position.y - origin_y], tex_coords: [1.0, 0.0], color: [color.r, color.g , color.b, color.a ] }); // bottom right

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