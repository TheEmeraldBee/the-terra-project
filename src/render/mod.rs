use wgpu::{CommandEncoder, TextureView};

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
    pub use super::vertex::Vertex;
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
}
