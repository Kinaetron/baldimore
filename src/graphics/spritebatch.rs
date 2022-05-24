use crate::{math::Vector2, platform::graphics_device::GraphicsDevice, platform::graphics_device::Vertex, graphics::texture::Texture};

pub struct SpriteBatch
{
    sprite_count: u32,
    batch_began: bool,
    texture_vec: Vec<Texture>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    graphics_device: GraphicsDevice,
}

impl SpriteBatch
{
    pub fn new(graphics_device: GraphicsDevice) -> Self
    {
        let vertices: Vec<Vertex> = Vec::new();
        let indices: Vec<u32> =Vec::new();
        let texture_vec: Vec<Texture> = Vec::new();

        Self { sprite_count: 0, batch_began: false, texture_vec, vertices, indices, graphics_device }
    }

    pub fn begin(&mut self) {

        if self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        self.batch_began = true;
    }


    pub fn draw(&mut self, texture: Texture, position: Vector2<f32>)
    {
        if !self.batch_began {
            panic!("You can't call begin twice in a row");
        }

        let origin_x = (texture.width / 2) as f32;
        let origin_y = (texture.height / 2) as f32;
        
        self.vertices.push( Vertex { position: [ position.x - origin_x, position.y + origin_y], tex_coords: [0.0, 0.0], texture_index: self.sprite_count });
        self.vertices.push( Vertex { position: [ position.x - origin_x, position.y - origin_y], tex_coords: [0.0, 1.0], texture_index: self.sprite_count });
        self.vertices.push( Vertex { position: [ position.x + origin_x, position.y - origin_y], tex_coords: [1.0, 1.0], texture_index: self.sprite_count });
        self.vertices.push( Vertex { position: [ position.x + origin_x, position.y + origin_y], tex_coords: [1.0, 0.0], texture_index: self.sprite_count });

        let indice_index = self.sprite_count;
        self.indices.push(indice_index);
        self.indices.push(indice_index + 1);
        self.indices.push(indice_index + 3);
        self.indices.push(indice_index + 1);
        self.indices.push(indice_index + 2);
        self.indices.push(indice_index + 3);

        self.texture_vec.push(texture);
        self.sprite_count += 1;
    }

    pub fn end(&mut self)
    {
        if !self.batch_began {
            panic!("You can't call end if without calling begin first");
        }



        self.batch_began = false;
    }
}