use std::any::Any;

use wgpu::{RenderPass, RenderPipeline, ShaderModule, ShaderStages};

use crate::{
    dir_light::DirectionalLight,
    light::{Light, Lights},
    renderer::Renderer,
    shader_tex::ShaderTexture,
    texture::Texture,
    uniform::Uniform,
};

pub trait Material: Any {
    fn update_uniforms(&mut self, renderer: &Renderer);
    fn apply(&mut self, render_pass: &mut RenderPass);
}

#[repr(C)]
#[derive(Default, bytemuck::Pod, Copy, Clone, bytemuck::Zeroable)]
pub struct VertexInput {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

pub struct DefaultMaterial {
    vertex_uniform: Uniform<VertexInput>,
    dir_light: Uniform<DirectionalLight>,
    lights: Lights,
    tex: ShaderTexture,

    pipeline: RenderPipeline,
}

impl DefaultMaterial {
    pub fn new(
        renderer: &Renderer,
        shader_module: &ShaderModule,
        dir_light: DirectionalLight,
        lights: &[Light],
        tex: Texture,
    ) -> Self {
        let (vertex_uniform, bind_group_layout) = Uniform::new(
            VertexInput::default(),
            renderer,
            0,
            ShaderStages::VERTEX | ShaderStages::FRAGMENT,
        );
        let (tex, tex_layout) = ShaderTexture::new(renderer, tex, 0);

        let (dir_light, dir_light_layout) =
            Uniform::new(dir_light, renderer, 0, ShaderStages::FRAGMENT);

        let (lights, layout) = Lights::new(lights, renderer).expect("Light count should be valid");

        let pipeline = renderer.pipeline(
            &[&bind_group_layout, &tex_layout, &dir_light_layout, &layout],
            shader_module,
        );

        Self {
            vertex_uniform,
            dir_light,
            lights,
            tex,
            pipeline,
        }
    }
}

impl Material for DefaultMaterial {
    fn update_uniforms(&mut self, renderer: &Renderer) {
        // Set the view matrix from the renderer's view matrix
        self.vertex_uniform.data.view_proj = renderer.view_matrix;
        self.vertex_uniform.data.view_position = renderer.view_pos;

        // Update the uniform with the new data.
        self.vertex_uniform.update(renderer);
    }

    fn apply(&mut self, render_pass: &mut RenderPass) {
        // Set the render pipeline
        render_pass.set_pipeline(&self.pipeline);

        // Apply the camera uniform
        self.vertex_uniform.apply(render_pass, 0);
        // Apply the texture
        self.tex.apply(render_pass, 1);

        self.dir_light.apply(render_pass, 2);

        self.lights.apply(render_pass, 3);
    }
}

pub struct UnlitMaterial {
    vertex_uniform: Uniform<VertexInput>,
    pipeline: RenderPipeline,
}

impl UnlitMaterial {
    pub fn new(renderer: &Renderer, shader_module: &ShaderModule) -> Self {
        let (vertex_uniform, bind_group_layout) =
            Uniform::new(VertexInput::default(), renderer, 0, ShaderStages::VERTEX);

        let pipeline = renderer.pipeline(&[&bind_group_layout], shader_module);

        Self {
            vertex_uniform,
            pipeline,
        }
    }
}

impl Material for UnlitMaterial {
    fn update_uniforms(&mut self, renderer: &Renderer) {
        // Set the view matrix from the renderer's view matrix
        self.vertex_uniform.data.view_proj = renderer.view_matrix;
        self.vertex_uniform.data.view_position = renderer.view_pos;

        // Update the uniform with the new data.
        self.vertex_uniform.update(renderer);
    }

    fn apply(&mut self, render_pass: &mut RenderPass) {
        // Set the render pipeline
        render_pass.set_pipeline(&self.pipeline);

        // Apply the camera uniform
        self.vertex_uniform.apply(render_pass, 0);
    }
}
