use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, RenderPass, ShaderStages};

use crate::renderer::Renderer;

pub struct Uniform<T: Zeroable + Pod> {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub data: T,
}

impl<T: Zeroable + Pod> Deref for Uniform<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Zeroable + Pod> DerefMut for Uniform<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Zeroable + Pod + Clone> Uniform<T> {
    pub fn new(
        data: T,
        renderer: &Renderer,
        binding: u32,
        visibility: ShaderStages,
    ) -> (Self, BindGroupLayout) {
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding,
                        visibility,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding,
                    resource: buffer.as_entire_binding(),
                }],
                label: None,
            });

        (
            Self {
                buffer,
                bind_group,
                data,
            },
            bind_group_layout,
        )
    }
    pub fn update(&mut self, renderer: &Renderer) {
        renderer
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]))
    }

    pub fn apply(&self, pass: &mut RenderPass, group: u32) {
        pass.set_bind_group(group, &self.bind_group, &[]);
    }
}
