use std::f32::consts::PI;

use app::{
    events::AppEvent,
    frame::UpdateFrame,
    scene::{Scene, SceneEvent},
    window_extension::WindowExtensions,
    App,
};
use glam::Vec3;
use noise::{NoiseFn, Perlin};
use render::{
    camera::Camera,
    frame::Frame,
    material::{DefaultMaterial, Material},
    mesh::{render::RenderMesh, Mesh},
    renderer::Renderer,
};
use wgpu::{include_wgsl, Color};
use winit::keyboard::KeyCode;
use world::{
    chunk::{Chunk, CHUNK_SIZE},
    tile::Tile,
};

fn main() -> anyhow::Result<()> {
    // Initialize the logger, filtering out spam logs.
    env_logger::Builder::from_default_env()
        .filter_module("wgpu_core::device::resource", log::LevelFilter::Error)
        .filter_module("wgpu_core::present", log::LevelFilter::Error)
        .filter_module("wgpu_core::device::life", log::LevelFilter::Error)
        .filter_module("wgpu_core::resource", log::LevelFilter::Error)
        .init();

    App::new(&TestScene::load).run()?;
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

        let material = DefaultMaterial::new(
            renderer,
            renderer
                .device
                .create_shader_module(include_wgsl!("../assets/shaders/basic.wgsl")),
        );

        let mut chunk = Chunk::new();
        let perlin = Perlin::new(0);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    const FREQUENCY: f64 = 0.05;
                    let value = perlin.get([
                        x as f64 * FREQUENCY,
                        y as f64 * FREQUENCY,
                        z as f64 * FREQUENCY,
                    ]);
                    if value >= 0.2 {
                        chunk.set([x, y, z], Some(Tile {}));
                    }
                }
            }
        }

        meshes.push(chunk.mesh(renderer));

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

            self.camera.pitch = self
                .camera
                .pitch
                .clamp((-PI / 2.0) + 0.01, (PI / 2.0) - 0.01);

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
        log::info!("Exiting Game!");
    }
}
