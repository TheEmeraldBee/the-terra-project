use glam::Vec3;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Color, RenderPass, ShaderStages};

use crate::renderer::Renderer;

#[derive(thiserror::Error, Debug)]
pub enum LightUniformError {
    #[error(
        "Light uniform has too many lights. Expected should be {MAX_LIGHT_COUNT}, but you had {0}"
    )]
    TooManyLights(usize),
}

const MAX_LIGHT_COUNT: usize = 256;

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

#[allow(unused)]
pub struct Lights {
    light_count_buffer: Buffer,
    light_buffer: Buffer,

    bind_group: BindGroup,
}

impl Lights {
    pub fn new(
        lights: &[Light],
        renderer: &Renderer,
    ) -> Result<(Self, BindGroupLayout), LightUniformError> {
        if lights.len() > 256 {
            return Err(LightUniformError::TooManyLights(lights.len()));
        }

        let light_count = lights.len();
        let mut res = [Light::default(); MAX_LIGHT_COUNT];
        res[..lights.len()].copy_from_slice(lights);

        let light_count_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[light_count]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let light_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Lights"),
                contents: bytemuck::cast_slice(&[res]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: light_count_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: light_buffer.as_entire_binding(),
                    },
                ],
                label: None,
            });

        Ok((
            Self {
                light_count_buffer,
                light_buffer,

                bind_group,
            },
            bind_group_layout,
        ))
    }

    pub fn apply(&self, pass: &mut RenderPass, group: u32) {
        pass.set_bind_group(group, &self.bind_group, &[]);
    }
}
