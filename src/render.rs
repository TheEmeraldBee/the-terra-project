use futures::executor::block_on;
use std::sync::Arc;
use thiserror::Error;
use wgpu::{CommandEncoder, CreateSurfaceError, Instance, RequestDeviceError, TextureView};
use winit::window::Window;

pub mod camera;

pub mod vertex;

#[derive(Error, Debug)]
pub enum RendererBuildError {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("Found no supported Adaptors")]
    RequestAdaptorError,

    #[error("Surface Unsupported By Adapter")]
    SurfaceConfigError,

    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}

pub struct Renderer<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<Window>, size: (u32, u32)) -> Result<Self, RendererBuildError> {
        block_on(Self::new_async(window, size))
    }

    async fn new_async(window: Arc<Window>, size: (u32, u32)) -> Result<Self, RendererBuildError> {
        let size = (size.0.max(1), size.1.max(1));
        let instance = Instance::default();

        let surface = instance.create_surface(window)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(RendererBuildError::RequestAdaptorError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await?;

        let config = surface
            .get_default_config(&adapter, size.0, size.1)
            .ok_or(RendererBuildError::SurfaceConfigError)?;
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            adapter,
        })
    }

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
