use wgpu::SurfaceError;
use crate::platform::system_sdl::SDLSystem;

pub struct GraphicsDevice
{
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GraphicsDevice
{
    pub fn new(sdl2_system: &SDLSystem) -> Result<Self, String>
    {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&sdl2_system.window) };

        let (width, height) = sdl2_system.window.size();

        let adapter_opt = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }));
        
            let adapter = match adapter_opt 
            {
                Some(a) => a,
                None => return Err(String::from("No adapter found")),
            };
        
            let (device, queue) = match pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor 
                {
                    limits: wgpu::Limits::default(),
                    label: Some("device"),
                    features: wgpu::Features::empty(),
                },
                None,
            )) {
                Ok(a) => a,
                Err(e) => return Err(e.to_string()),
            };

            let config = wgpu::SurfaceConfiguration 
            {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            };

            surface.configure(&device, &config);

        Ok(Self{ surface, device, queue, config })
    }

    pub fn clear_colour(&self, red: f64, green: f64, blue: f64, alpha: f64)
    {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => 
            {
                let reason = match err 
                {
                    SurfaceError::Timeout => "Timeout",
                    SurfaceError::Outdated => "Outdated",
                    SurfaceError::Lost => "Lost",
                    SurfaceError::OutOfMemory => "OutOfMemory",
                };
                panic!("Failed to get current surface texture! Reason: {}", reason)
            }
        };

        let output = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor 
        {
            color_attachments: &[wgpu::RenderPassColorAttachment 
            {
                view: &output,
                resolve_target: None,
                ops: wgpu::Operations 
                {
                    load: wgpu::LoadOp::Clear( wgpu::Color {r: red, g: green, b: blue, a: alpha }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
            label: None,
        });


        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}