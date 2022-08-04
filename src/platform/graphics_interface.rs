use log::warn;
use wgpu::{util::DeviceExt, Sampler, TextureView};
use crate::math;
use cgmath::{Matrix4};
use std::num::NonZeroU32;
use crate::platform::system_sdl::SDLSystem;
use std::{iter, sync::Arc};
use crate::{graphics::texture::Texture};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub index: u32,
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4]
}

pub struct GraphicsInterface
{
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface,
    pub world_matrix: Matrix4<f32>,
    pub config: wgpu::SurfaceConfiguration,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,
}

impl GraphicsInterface
{
    pub fn new(sdl2_system: &SDLSystem) -> Result<Self, String>
    {
        let instance = wgpu::Instance::new(wgpu::Backends::DX12);
        let surface = unsafe { instance.create_surface(&sdl2_system.window) };

        let (width, height) = sdl2_system.window.size();

        let adapter_opt = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions 
        {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }));
        
        let adapter = match adapter_opt 
        {
            Some(a) => a,
            None => return Err(String::from("No adapter found")),
        };

        let adapter_features = adapter.features();

        if ! adapter_features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY) {
            return Err(String::from("Texture Binding isn't supported !"));
        }

        if !adapter_features.contains(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING) {
            return Err(String::from("Sampled Texture and Storage Buffer Array Non Uniform Indexing isn't supported !"));
        }

        let (device, queue) = match pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                label: Some("device"),
                limits: wgpu::Limits::default(),
                features:  wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | wgpu::Features::TEXTURE_BINDING_ARRAY
            },
            None,
        )) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };

        let config = wgpu::SurfaceConfiguration 
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        surface.configure(&device, &config);
            
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        
        let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: NonZeroU32::new(16),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(16),
                    },
                ],
            label: Some("texture_bind_group_layout"),
        });
       

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Sint32, 1 => Float32x2, 2 => Float32x2, 3 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let clear_color = wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let world_matrix = math::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        
        Ok(Self{ surface, device, queue, config, texture_bind_group_layout, render_pipeline, clear_color, world_matrix })
    }

    pub fn clear(&mut self, red : f64, green: f64, blue: f64, alpha: f64) {
        self.clear_color = wgpu::Color { r: red, g: green, b: blue, a: alpha };
    }

    pub fn batch_render(& mut self, textures: &Vec<Arc<Texture>>, vertices: &Vec<Vertex>, indices: &Vec<u16>)
    { 
        match self.internal_batch_render(textures, vertices, indices) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.surface.configure(&self.device, &self.config),
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("System has run out of memory"),
            Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
        }
    }

    fn internal_batch_render(& mut self, textures: &Vec<Arc<Texture>>,  vertices: &Vec<Vertex>, indices: &Vec<u16>) -> Result<(), wgpu::SurfaceError>
    {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        let mut texture_view_vec: Vec<&TextureView> = Vec::with_capacity(textures.len());
        let mut texture_sampler_vec: Vec<&Sampler> = Vec::with_capacity(textures.len());

        for texture in textures
        {
            texture_view_vec.push(&texture.view);
            texture_sampler_vec.push(&texture.sampler);
        }

        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[
                wgpu::BindGroupEntry 
                {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(texture_view_vec.as_slice()),
                },
                wgpu::BindGroupEntry 
                {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(texture_sampler_vec.as_slice()),
                }
            ],
            layout: &self.texture_bind_group_layout,
            label: Some("texture bind group"),
        });
        
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));  
        output.present(); 

        Ok(())
    }

}