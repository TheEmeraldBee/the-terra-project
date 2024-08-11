use std::f32::consts::PI;

use glam::Vec3;
use render::{camera::Camera, vertex::Vertex};
use scene::{Scene, SceneEvent};
use wgpu::{util::DeviceExt, BindGroup, Buffer, RenderPipeline};
use window::Frame;
use winit::{event::WindowEvent, event_loop::EventLoop, keyboard::KeyCode};

pub mod window;
use crate::window::App;

pub mod render;

pub mod scene;

pub mod input;

pub mod time;

pub mod math;

fn main() -> anyhow::Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Create an event loop.
    let event_loop: EventLoop<()> = EventLoop::builder().build()?;

    event_loop.run_app(&mut App::new(&TestScene::load))?;
    Ok(())
}

const VERTICES: &[Vertex] = &[
    Vertex {
        // 0
        position: [0.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        // 1
        position: [1.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        // 2
        position: [1.0, 1.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        // 3
        position: [0.0, 1.0, 0.0],
        color: [1.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub struct TestScene {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,
    index_buffer: Buffer,

    num_indices: u32,

    camera: Camera,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,

    bound: bool,
}

impl TestScene {
    fn load(window: &winit::window::Window, renderer: &render::Renderer) -> Box<dyn Scene> {
        let camera = Camera {
            pos: Vec3::new(0.0, 1.0, -5.0),
            ..Default::default()
        };

        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .unwrap();
        window.set_cursor_visible(false);

        // Generate Buffers
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        let camera_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera.uniform()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let camera_bind_group_layout =
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

        let camera_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        let shader = renderer
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../assets/shaders/basic.wgsl").into(),
                ),
            });

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let swapchain_capabilities = renderer.surface.get_capabilities(&renderer.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let pipeline = renderer
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        Box::new(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,

            camera,
            camera_buffer,
            camera_bind_group,

            bound: true,
        })
    }
}

impl Scene for TestScene {
    fn update(&mut self, frame: Frame) -> SceneEvent {
        let renderer = frame.renderer;
        let input = frame.input;
        let time = frame.time;
        let delta = time.delta_seconds();

        // Update Camera Projection Matrix
        self.camera.update_view();
        let proj = self.camera.uniform();
        renderer
            .queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[proj]));

        if frame.input.just_pressed(winit::keyboard::KeyCode::Escape) {
            self.bound = !self.bound;
            match self.bound {
                true => {
                    frame
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                        .unwrap();
                    frame.window.set_cursor_visible(false);
                }
                false => {
                    frame
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::None)
                        .unwrap();
                    frame.window.set_cursor_visible(true);
                }
            }
        }

        if self.bound {
            let mouse_delta = input.mouse_delta();

            self.camera.yaw += mouse_delta.x * 0.5 * delta;
            self.camera.pitch -= mouse_delta.y * 0.5 * delta;

            self.camera.pitch = self.camera.pitch.clamp(-PI / 2.0, PI / 2.0);
            self.camera.yaw = self.camera.yaw.rem_euclid(2.0 * PI);
        }

        let f = self.camera.forward();
        let r = self.camera.right();
        let u = Vec3::Y;

        if input.pressed(KeyCode::KeyW) {
            self.camera.pos += f * 5.0 * delta;
        }
        if input.pressed(KeyCode::KeyS) {
            self.camera.pos -= f * 5.0 * delta;
        }
        if input.pressed(KeyCode::KeyD) {
            self.camera.pos += r * 5.0 * delta;
        }
        if input.pressed(KeyCode::KeyA) {
            self.camera.pos -= r * 5.0 * delta;
        }
        if input.pressed(KeyCode::Space) {
            self.camera.pos += u * 5.0 * delta;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            self.camera.pos -= u * 5.0 * delta;
        }

        renderer.frame(|view, encoder| {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the render pipeline
            pass.set_pipeline(&self.pipeline);

            // Set the camera's bind group
            pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Set the vertex buffer.
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // Set the index buffer.
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.

            // Draw the vertices.
            pass.draw_indexed(0..self.num_indices, 0, 0..1);
        });
        SceneEvent::Empty
    }

    #[allow(clippy::single_match)]
    fn event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(s) => self.camera.resize(s),
            _ => {}
        }
    }
}
