use crate::{math::Vector2, platform::graphics_device::GraphicsDevice, platform::graphics_device::Vertex, graphics::texture::Texture};

pub struct SpriteBatch
{
    batch_began: bool,
    pub graphics_device: GraphicsDevice,
    batch_information_vec: Vec<BatchInformation>
}


#[derive(Clone)]
pub struct BatchInformation
{
    pub texture: Texture,
    pub indices: Vec<u16>,
    pub vertices: Vec<Vertex>,
}

impl BatchInformation
{
    pub fn new(texture: Texture, vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { texture, indices, vertices }
    }
}

impl SpriteBatch
{
    pub fn new(graphics_device: GraphicsDevice) -> Self 
    {
        let batch_information_vec: Vec<BatchInformation> = Vec::new();

        Self { batch_began: false, graphics_device, batch_information_vec }
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

        let mut vertices: Vec<Vertex>  = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        vertices.push(Vertex { position: [ position.x - origin_x, position.y - origin_y], tex_coords: [0.0, 0.0] }); // bottom left
        vertices.push(Vertex { position: [ position.x - origin_x, position.y + origin_y], tex_coords: [0.0, 1.0] }); // top left
        vertices.push(Vertex { position: [ position.x + origin_x, position.y + origin_y], tex_coords: [1.0, 1.0] }); // top right
        vertices.push(Vertex { position: [ position.x + origin_x, position.y - origin_y], tex_coords: [1.0, 0.0] }); // bottom right

        indices.push(0);
        indices.push(1);
        indices.push(3);
        indices.push(1);
        indices.push(2);
        indices.push(3);

        let batch_information = BatchInformation::new(texture, vertices, indices);
        self.batch_information_vec.push(batch_information);
    }

    pub fn end(&mut self)
    {
        if !self.batch_began {
            panic!("You can't call end if without calling begin first");
        }

        self.graphics_device.batch_render(self.batch_information_vec.clone());
        self.batch_began = false;
    }
}