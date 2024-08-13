use wgpu::{
    CommandEncoder, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
    TextureView,
};
use winit::window::Window;

use crate::renderer::Renderer;

pub struct Frame<'a, 'r, 'e> {
    pub renderer: &'a Renderer<'r>,
    pub window: &'a Window,
    pub view: &'a TextureView,

    pub encoder: &'e mut CommandEncoder,
}

impl<'a, 'r, 'e> Frame<'a, 'r, 'e> {
    pub fn pass(&mut self, clear_color: wgpu::Color) -> RenderPass<'_> {
        self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.renderer.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }
}
