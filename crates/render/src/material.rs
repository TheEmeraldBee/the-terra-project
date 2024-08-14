use std::any::Any;

use glam::Vec3;
use wgpu::{Color, RenderPass, RenderPipeline, ShaderModule, ShaderStages};

use crate::{renderer::Renderer, shader_tex::ShaderTexture, texture::Texture, uniform::Uniform};

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

#[repr(C)]
#[derive(Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 3],
    // Padding to align to 4 bytes
    _p: u32,
    pub color: [f32; 3],
    // Padding to align to 4 bytes
    _p2: u32,
}

impl Light {
    pub fn new(pos: Vec3, color: Color) -> Self {
        Self {
            position: pos.to_array(),
            _p: 0,
            color: [color.r as f32, color.g as f32, color.b as f32],
            _p2: 0,
        }
    }
}

pub struct DefaultMaterial {
    vertex_uniform: Uniform<VertexInput>,
    light_count_uniform: Uniform<i32>,
    lighting_uniform: Uniform<[Light; 3]>,
    tex: ShaderTexture,

    pipeline: RenderPipeline,
}

impl DefaultMaterial {
    pub fn new(
        renderer: &Renderer,
        shader_module: &ShaderModule,
        lights: [Light; 3],
        tex: Texture,
    ) -> Self {
        let (vertex_uniform, bind_group_layout) = Uniform::new(
            VertexInput::default(),
            renderer,
            0,
            ShaderStages::VERTEX | ShaderStages::FRAGMENT,
        );
        let (tex, tex_layout) = ShaderTexture::new(renderer, tex, 0);

        let (light_count_uniform, light_count_bind_group_layout) =
            Uniform::new(3, renderer, 0, ShaderStages::FRAGMENT);
        let (lighting_uniform, light_bind_group_layout) =
            Uniform::new(lights, renderer, 0, ShaderStages::FRAGMENT);

        let pipeline = renderer.pipeline(
            &[
                &bind_group_layout,
                &tex_layout,
                &light_count_bind_group_layout,
                &light_bind_group_layout,
            ],
            shader_module,
        );

        Self {
            vertex_uniform,
            lighting_uniform,
            light_count_uniform,
            tex,
            pipeline,
        }
    }

    // pub fn change_light(&mut self, renderer: &Renderer, light: Light) {
    //     self.lighting_uniform.data = light;
    //     self.lighting_uniform.update(renderer);
    // }
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

        self.light_count_uniform.apply(render_pass, 2);

        // Apply the light
        self.lighting_uniform.apply(render_pass, 3);
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
