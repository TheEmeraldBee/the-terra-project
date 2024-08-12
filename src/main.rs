use std::f32::consts::PI;

use app::window_extension::WindowExtensions;
use bytemuck::{Pod, Zeroable};
use mesh::Mesh;
use prelude::*;
use wgpu::ShaderStages;
use winit::{dpi::PhysicalSize, event_loop::EventLoop};

pub mod app;
use crate::app::App;

pub mod render;

pub mod scene;

pub mod input;

pub mod time;

pub mod prelude;

pub mod mesh;

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
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        // 1
        position: [15.0, 0.0, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        // 2
        position: [15.0, 15.0, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        // 3
        position: [0.0, 15.0, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub struct TestScene {
    pipeline: RenderPipeline,

    mesh: Mesh,

    depth_texture: Texture,

    camera: Camera,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,

    u_resolution: UResolution,
    u_res_buffer: Buffer,
    u_res_bind_group: BindGroup,

    bound: bool,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Zeroable)]
pub struct UResolution {
    size: Vec2,
}

impl UResolution {
    pub fn new(physical_size: (f32, f32)) -> Self {
        Self {
            size: Vec2::new(physical_size.0, physical_size.1),
        }
    }

    pub fn update(&mut self, size: PhysicalSize<u32>) {
        self.size.x = size.width as f32;
        self.size.y = size.height as f32;
    }
}

impl TestScene {
    fn load(window: &winit::window::Window, renderer: &Renderer) -> Box<dyn Scene> {
        let camera = Camera {
            pos: Vec3::new(0.0, 1.0, -5.0),
            ..Default::default()
        };

        window.lock_cursor(true);

        let (camera_buffer, camera_bind_group_layout, camera_bind_group) =
            camera.bind_group(renderer);

        let inner = window.inner_size();
        let u_resolution = UResolution::new((inner.width as f32, inner.height as f32));

        let (u_res_buffer, u_res_bind_group_layout, u_res_bind_group) =
            renderer.uniform(u_resolution, ShaderStages::FRAGMENT, 0);

        let shader = renderer
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../assets/shaders/basic.wgsl").into(),
                ),
            });

        let pipeline = Mesh::pipeline(
            PrimitiveTopology::TriangleList,
            renderer,
            shader,
            &[&u_res_bind_group_layout, &camera_bind_group_layout],
        );
        let mesh = Mesh::new(renderer, VERTICES, INDICES);

        let depth_texture =
            Texture::create_depth_texture(&renderer.device, &renderer.config, "depth_texture");

        Box::new(Self {
            pipeline,
            mesh,

            depth_texture,

            camera,
            camera_buffer,
            camera_bind_group,

            u_resolution,
            u_res_buffer,
            u_res_bind_group,

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

        // Toggle Cursor Lock On ESC Key Pressed
        if frame.input.just_pressed(winit::keyboard::KeyCode::Escape) {
            self.bound = !self.bound;
            frame.window.lock_cursor(self.bound);
        }

        // If the mouse is locked, rotate the camera.
        if self.bound {
            let mouse_delta = input.mouse_delta();

            self.camera.yaw += mouse_delta.x * 0.5 * delta;
            self.camera.pitch -= mouse_delta.y * 0.5 * delta;

            self.camera.pitch = self.camera.pitch.clamp(-PI / 2.0, PI / 2.0);
            self.camera.yaw = self.camera.yaw.rem_euclid(2.0 * PI);
        }

        // Move camera based on input.
        let f = self.camera.forward();
        let r = self.camera.right();
        let u = Vec3::Y;

        let horizontal =
            input.key_vector(KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD);
        let vert = input.key_value(KeyCode::Space, KeyCode::ShiftLeft);

        self.camera.pos += f * horizontal.y * 5.0 * delta;
        self.camera.pos += r * horizontal.x * 5.0 * delta;
        self.camera.pos += u * vert * 5.0 * delta;

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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the render pipeline
            pass.set_pipeline(&self.pipeline);

            // Set the camera's bind group
            pass.set_bind_group(1, &self.camera_bind_group, &[]);
            // Set the u_resolution's bind group
            pass.set_bind_group(0, &self.u_res_bind_group, &[]);

            self.mesh.render(&mut pass);
        });
        SceneEvent::Empty
    }

    #[allow(clippy::single_match)]
    fn event(&mut self, event: &WindowEvent, renderer: &Renderer, _window: &Window) {
        match event {
            WindowEvent::Resized(s) => {
                self.camera.resize(s);
                self.depth_texture = Texture::create_depth_texture(
                    &renderer.device,
                    &renderer.config,
                    "depth_texture",
                );
                self.u_resolution.update(*s);
                renderer.queue.write_buffer(
                    &self.u_res_buffer,
                    0,
                    bytemuck::cast_slice(&[self.u_resolution]),
                )
            }
            _ => {}
        }
    }
}
