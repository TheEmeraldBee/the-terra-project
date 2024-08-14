use wgpu::{BindGroup, BindGroupLayout, RenderPass};

use crate::{renderer::Renderer, texture::Texture};

pub struct ShaderTexture {
    pub texture: Texture,

    pub bind_group: BindGroup,
}

impl ShaderTexture {
    pub fn new(renderer: &Renderer, tex: Texture, bind_start: u32) -> (Self, BindGroupLayout) {
        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: bind_start,
                        resource: wgpu::BindingResource::TextureView(&tex.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: bind_start + 1,
                        resource: wgpu::BindingResource::Sampler(&tex.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

        (
            Self {
                texture: tex,
                bind_group,
            },
            layout,
        )
    }

    pub fn apply(&self, pass: &mut RenderPass, group: u32) {
        pass.set_bind_group(group, &self.bind_group, &[]);
    }
}
