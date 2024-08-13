use wgpu::RenderPass;

use super::Mesh;

pub trait RenderMesh {
    fn render_mesh(&mut self, mesh: &Mesh);
}

impl<'pass> RenderMesh for RenderPass<'pass> {
    fn render_mesh(&mut self, mesh: &Mesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_indices, 0, 0..1);
    }
}
