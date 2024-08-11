use glam::*;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer};
use winit::dpi::PhysicalSize;

use super::{projection::Projection, Renderer};

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    pub projection: Projection,

    pub uniform: CameraUniform,
}

impl Default for Camera {
    fn default() -> Self {
        let mut res = Self {
            pos: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,

            projection: Projection::default(),

            uniform: CameraUniform::default(),
        };
        res.update_view();
        res
    }
}

impl Camera {
    pub fn update_view(&mut self) {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let view = Mat4::look_to_rh(
            self.pos,
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::Y,
        );

        self.uniform.update(self.projection.proj * view);
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.projection.resize(size)
    }

    pub fn uniform(&self) -> CameraUniform {
        self.uniform
    }

    pub fn forward(&self) -> Vec3 {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        Vec3::new(yaw_cos, 0.0, yaw_sin).normalize()
    }

    pub fn right(&self) -> Vec3 {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize()
    }

    pub fn bind_group(&self, renderer: &Renderer) -> (Buffer, BindGroupLayout, BindGroup) {
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[self.uniform()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        (buffer, bind_group_layout, bind_group)
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

impl CameraUniform {
    pub fn update(&mut self, proj: Mat4) {
        self.view_proj = proj.to_cols_array_2d()
    }
}
