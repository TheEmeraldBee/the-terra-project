use bytemuck::Pod;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, ShaderStages, TextureView,
};

pub mod camera;

pub mod vertex;

pub mod builder;

pub mod error;

pub mod bind_group;

pub mod projection;

pub mod texture;

pub mod prelude {
    pub use super::builder::NewRenderer;
    pub use super::camera::Camera;
    pub use super::error::RendererBuildError;
    pub use super::texture::Texture;
    pub use super::vertex::{vertex, Vertex};
    pub use super::Renderer;
}

pub struct Renderer<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}

impl<'a> Renderer<'a> {
    pub fn resize(&mut self, new_size: (u32, u32)) {
        // Reconfigure the surface with the new size
        self.config.width = new_size.0.max(1);
        self.config.height = new_size.1.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn frame<T>(&self, f: impl Fn(&TextureView, &mut CommandEncoder) -> T) -> T {
        // Get the current surface texture.
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        // Create frame view out of current texture.
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Run Passed Rendering Function.
        let res = f(&view, &mut encoder);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        res
    }

    pub fn uniform<T: Pod>(
        &self,
        buffer_data: T,
        visibility: ShaderStages,
        binding: u32,
    ) -> (Buffer, BindGroupLayout, BindGroup) {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[buffer_data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding,
                        visibility,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        (buffer, bind_group_layout, bind_group)
    }
}
