use crate::prelude::*;
use projection::Projection;
use wgpu::{
    BindGroupLayout, CommandEncoder, FragmentState, MultisampleState, PipelineCompilationOptions,
    PrimitiveState, RenderPipelineDescriptor, ShaderModule, SurfaceTexture, TextureView,
    VertexState,
};
use winit::dpi::PhysicalSize;

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

    pub depth_texture: Texture,
    pub projection: Projection,
    pub view_matrix: [[f32; 4]; 4],
}

impl<'a> Renderer<'a> {
    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        // Reconfigure the surface with the new size.
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.config);

        // Re-build the depth-texture and projection matrix.
        self.depth_texture =
            Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        self.projection.resize(size);
    }

    pub fn frame(&self) -> (SurfaceTexture, TextureView, CommandEncoder) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        // Create frame view out of current texture.
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        (frame, view, encoder)
    }

    pub fn pipeline(
        &self,
        bind_group_layouts: &[&BindGroupLayout],
        shader_module: &ShaderModule,
    ) -> RenderPipeline {
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts,
                    push_constant_ranges: &[],
                });

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: shader_module,
                    entry_point: "vs_main",
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: shader_module,
                    entry_point: "fs_main",
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                multisample: MultisampleState::default(),
                multiview: None,
                cache: None,
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilState::default(),     // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),
            })
    }
}
