use std::ops::Add;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

impl Add<[f32; 3]> for Vertex {
    type Output = Vertex;
    fn add(mut self, rhs: [f32; 3]) -> Self {
        self.position[0] += rhs[0];
        self.position[1] += rhs[1];
        self.position[2] += rhs[2];

        self
    }
}

pub const fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    Vertex {
        position: [x, y, z],
    }
}
