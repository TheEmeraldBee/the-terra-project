use std::any::Any;

use wgpu::{RenderPass, RenderPipeline, ShaderModule, ShaderStages};

use crate::{renderer::Renderer, uniform::Uniform};

pub trait Material: Any {
    fn update_uniforms(&mut self, renderer: &Renderer);
    fn apply(&mut self, render_pass: &mut RenderPass);
}

#[repr(C)]
#[derive(Default, bytemuck::Pod, Copy, Clone, bytemuck::Zeroable)]
pub struct VertexInput {
    view_proj: [[f32; 4]; 4],
}

pub struct DefaultMaterial {
    uniform: Uniform<VertexInput>,

    pipeline: RenderPipeline,
}

impl DefaultMaterial {
    pub fn new(renderer: &Renderer, shader_module: ShaderModule) -> Self {
        let (uniform, bind_group_layout) =
            Uniform::new(VertexInput::default(), renderer, 0, ShaderStages::VERTEX);

        let pipeline = renderer.pipeline(&[&bind_group_layout], &shader_module);

        Self { uniform, pipeline }
    }
}

impl Material for DefaultMaterial {
    fn update_uniforms(&mut self, renderer: &Renderer) {
        // Set the view matrix from the renderer's view matrix
        self.uniform.data.view_proj = renderer.view_matrix;

        // Update the uniform with the new data.
        self.uniform.update(renderer);
    }

    fn apply(&mut self, render_pass: &mut RenderPass) {
        // Set the render pipeline
        render_pass.set_pipeline(&self.pipeline);

        // Apply the camera uniform
        self.uniform.apply(render_pass, 0);
    }
}
