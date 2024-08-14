use crate::{error::RendererBuildError, projection::Projection};

use super::Renderer;
use futures::executor::block_on;
use std::sync::Arc;
use wgpu::Instance;
use winit::window::Window;

pub trait NewRenderer<'a> {
    fn build(window: Arc<Window>, size: (u32, u32)) -> Result<Renderer<'a>, RendererBuildError>;

    #[allow(async_fn_in_trait)]
    async fn build_async(
        window: Arc<Window>,
        size: (u32, u32),
    ) -> Result<Renderer<'a>, RendererBuildError>;
}

impl<'a> NewRenderer<'a> for Renderer<'a> {
    fn build(window: Arc<Window>, size: (u32, u32)) -> Result<Self, RendererBuildError> {
        block_on(Self::build_async(window, size))
    }

    async fn build_async(
        window: Arc<Window>,
        size: (u32, u32),
    ) -> Result<Self, RendererBuildError> {
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

        let depth_texture = super::Texture::create_depth_texture(&device, &config, "Depth Texture");

        Ok(Self {
            surface,
            device,
            queue,
            config,
            adapter,

            depth_texture,

            projection: Projection::default(),
            view_matrix: [[0.0; 4]; 4],
            view_pos: [0.0; 4],
        })
    }
}
