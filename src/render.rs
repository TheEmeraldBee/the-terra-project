use futures::executor::block_on;
use std::sync::Arc;
use thiserror::Error;
use wgpu::{
    CommandEncoder, CreateSurfaceError, Instance, Queue, RenderPipeline, RequestDeviceError,
    ShaderModuleDescriptor, ShaderSource, TextureView,
};
use winit::window::Window;

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
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
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

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn render_pipeline(&self, source: &str) -> RenderPipeline {
        let descriptor = wgpu::ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(source)),
        };
        self.render_pipeline_descriptor(descriptor)
    }

    pub fn render_pipeline_descriptor(&self, source: ShaderModuleDescriptor) -> RenderPipeline {
        let shader = self.device.create_shader_module(source);

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
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
