use std::fs;
use std::path::Path;
use cgmath::Vector2; 
use std::num::NonZeroU32;
use image::{GenericImageView, ImageBuffer, Rgba};
use crate::platform::graphics_interface::GraphicsInterface;


pub struct Texture
{
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture 
{
    pub fn new_from_buffer(graphics_interface: &GraphicsInterface, buffer: ImageBuffer<Rgba<u8>, Vec<u8>>, dimensions: Vector2<u32>) -> Self
    {
        let size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };

        let texture = graphics_interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("image from buffer"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        graphics_interface.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.x),
                rows_per_image: NonZeroU32::new(dimensions.y),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = graphics_interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { id: rand::random::<u64>(), width: dimensions.x, height: dimensions.y, texture, view, sampler }
    }


    pub fn new_from_file(graphics_interface: &GraphicsInterface, file_path: &str) -> Self
    {
       
        let path = Path::new(file_path);
        let data = fs::read(path).unwrap();
        
        let image = image::load_from_memory(&data).unwrap();
        let rgba = image.to_rgba8();

        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = graphics_interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("image from file path"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        graphics_interface.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = graphics_interface.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { id: rand::random::<u64>(), width: dimensions.0, height: dimensions.1 , texture, view, sampler }
    }

    pub fn new_glpyh(graphics_interface: &GraphicsInterface, bitmap: &Vec<u8>, dimensions: &Vector2<u32>) -> Self
    {
        let size = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth_or_array_layers: 1,
        };

        let texture = graphics_interface.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyth from memory"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = graphics_interface.device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Nearest,
            mag_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0f32,
            lod_max_clamp: 0f32,
            ..Default::default()
        });

        graphics_interface.queue.write_texture(
            wgpu::ImageCopyTexture 
            {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &bitmap,
            wgpu::ImageDataLayout 
            {
                offset: 0,
                bytes_per_row: NonZeroU32::new(dimensions.x),
                rows_per_image: NonZeroU32::new(dimensions.y),
            },
            size,
        );

        Self { id: rand::random::<u64>(), width: dimensions.x, height: dimensions.y, texture, view, sampler }
    }
}