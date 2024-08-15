use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::Color;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DirectionalLight {
    dir: [f32; 3],
    _p0: u32,
    color: [f32; 3],
    _p1: u32,
}

impl DirectionalLight {
    pub fn new(dir: Vec3, color: Color) -> Self {
        Self {
            dir: dir.normalize().to_array(),
            _p0: 0,
            color: [color.r as f32, color.g as f32, color.b as f32],
            _p1: 0,
        }
    }
}
