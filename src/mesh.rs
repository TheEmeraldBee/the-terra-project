use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderModule,
};

use crate::prelude::{vertex, Renderer, Texture, Vertex};

#[derive(Default)]
pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    face_count: u32,
}

impl MeshBuilder {
    pub fn add(&mut self, coord: [f32; 3], face: usize) {
        // Push all vertice faces
        for i in &VERTICES[face] {
            self.vertices.push(*i + coord)
        }

        for i in &INDICES {
            self.indices.push(*i + (4 * self.face_count))
        }

        self.face_count += 1;
    }

    pub fn build(self, renderer: &Renderer) -> Mesh {
        Mesh::new(renderer, &self.vertices, &self.indices)
    }
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
                    label: Some("Render Pipeline Layout"),
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

    pub fn render(&self, pass: &mut RenderPass<'_>) {
        // Set the vertex buffer.
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // Set the index buffer.
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.

        // Draw the vertices.
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

const INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];
const VERTICES: [[Vertex; 4]; 6] = [
    [
        // Top
        vertex(0.0, 1.0, 0.0),
        vertex(0.0, 1.0, 1.0),
        vertex(1.0, 1.0, 0.0),
        vertex(1.0, 1.0, 1.0),
    ],
    [
        // Bottom
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 0.0, 0.0),
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 0.0, 0.0),
    ],
    [
        // Left
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 1.0, 1.0),
        vertex(0.0, 0.0, 0.0),
        vertex(0.0, 1.0, 0.0),
    ],
    [
        // Right
        vertex(1.0, 0.0, 0.0),
        vertex(1.0, 1.0, 0.0),
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 1.0, 1.0),
    ],
    [
        // Front
        vertex(1.0, 0.0, 0.0),
        vertex(1.0, 1.0, 0.0),
        vertex(0.0, 0.0, 0.0),
        vertex(0.0, 1.0, 0.0),
    ],
    [
        // Back
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 1.0, 1.0),
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 1.0, 1.0),
    ],
];
