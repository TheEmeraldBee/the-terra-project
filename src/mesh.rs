use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PrimitiveState, PrimitiveTopology, RenderPipeline,
    ShaderModule,
};

use crate::prelude::{Renderer, Texture, Vertex};

pub mod builder;
pub mod render;

pub mod prelude {
    pub use super::builder::MeshBuilder;
    pub use super::render::RenderMesh;
    pub use super::Mesh;
}

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
}

impl Mesh {
    pub fn new(renderer: &Renderer, vertices: &[Vertex], indices: &[u32]) -> Self {
        // Generate Buffers
        let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn pipeline(
        topology: PrimitiveTopology,
        renderer: &Renderer,
        shader: ShaderModule,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts,
                    push_constant_ranges: &[],
                });

        let swapchain_capabilities = renderer.surface.get_capabilities(&renderer.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        renderer
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: PrimitiveState {
                    topology,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilState::default(),     // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            })
    }
}
