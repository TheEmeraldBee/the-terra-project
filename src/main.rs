use std::f32::consts::PI;

use app::{
    events::AppEvent,
    frame::UpdateFrame,
    scene::{Scene, SceneEvent},
    window_extension::WindowExtensions,
    App,
};
use glam::Vec3;
use render::{
    camera::Camera,
    frame::Frame,
    material::{DefaultMaterial, Material},
    mesh::{builder::MeshBuilder, render::RenderMesh, Mesh},
    renderer::Renderer,
};
use wgpu::{include_wgsl, Color};
use winit::{event_loop::EventLoop, keyboard::KeyCode};

fn main() -> anyhow::Result<()> {
    // Initialize the logger.
    env_logger::init();

    // Create an event loop.
    let event_loop: EventLoop<()> = EventLoop::builder().build()?;

    event_loop.run_app(&mut App::new(&TestScene::load))?;
    Ok(())
}

pub struct TestScene {
    meshes: Vec<Mesh>,

    camera: Camera,

    bound: bool,

    material: DefaultMaterial,
}

impl TestScene {
    fn load(window: &winit::window::Window, renderer: &Renderer) -> Box<dyn Scene> {
        let camera = Camera {
            pos: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        };

        let mut meshes = vec![];

        window.lock_cursor(true);

        // Generate Meshes for example scene.
        for iteration in 0..12 {
            let mut mesh = MeshBuilder::default();
            for face in 0..6 {
                mesh.add([0.0, 0.0, -5.0], face);
            }

            meshes.push(
                mesh.with_translation([0.0, 0.0, iteration as f32 * 3.0])
                    .build(renderer),
            )
        }

        let material = DefaultMaterial::new(
            renderer,
            renderer
                .device
                .create_shader_module(include_wgsl!("../assets/shaders/basic.wgsl")),
        );

        Box::new(Self {
            meshes,

            camera,

            bound: true,

            material,
        })
    }
}

impl Scene for TestScene {
    fn update(&mut self, frame: &mut UpdateFrame) -> SceneEvent {
        let input = frame.input;
        let time = frame.time;
        let delta = time.delta_seconds();

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

        let mut vel = Vec3::ZERO;

        let horizontal =
            input.key_vector(KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD);
        let vert = input.key_value(KeyCode::Space, KeyCode::ShiftLeft);

        vel += f * horizontal.y;
        vel += r * horizontal.x;
        vel += u * vert;

        if input.pressed(KeyCode::ControlLeft) {
            vel *= 10.0;
        } else {
            vel *= 5.0;
        }
        vel *= delta;
        self.camera.pos += vel;

        frame.events.register(AppEvent::ApplyCamera(self.camera));

        SceneEvent::Empty
    }

    fn render(&mut self, frame: &mut Frame) {
        // Update Uniforms
        self.material.update_uniforms(frame.renderer);

        // Create Pass
        let mut pass = frame.pass(Color::BLACK);
        // Apply Materials To Pass
        self.material.apply(&mut pass);

        // Render Meshes.
        for mesh in &self.meshes {
            pass.render_mesh(mesh);
        }
    }

    fn exit(&mut self) {
        println!("Thank you for playing!");
    }
}
