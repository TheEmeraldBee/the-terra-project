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
    dir_light::DirectionalLight,
    frame::Frame,
    light::Light,
    material::{DefaultMaterial, Material, UnlitMaterial},
    mesh::{builder::MeshBuilder, render::RenderMesh, Mesh},
    renderer::Renderer,
    texture::Texture,
    util::color,
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
    unlit_meshes: Vec<Mesh>,

    camera: Camera,

    bound: bool,

    material: DefaultMaterial,
    unlit_material: UnlitMaterial,
}

impl TestScene {
    fn load(window: &winit::window::Window, renderer: &Renderer) -> Box<dyn Scene> {
        let camera = Camera {
            pos: Vec3::new(0.0, CHUNK_SIZE as f32 + 10.0, 0.0),
            pitch: -PI / 2.0,
            ..Default::default()
        };

        let mut meshes = vec![];
        let mut unlit_meshes = vec![];

        window.lock_cursor(true);

        let material = DefaultMaterial::new(
            renderer,
            &renderer
                .device
                .create_shader_module(include_wgsl!("../assets/shaders/basic.wgsl")),
            DirectionalLight::new(Vec3::new(0.0, 1.0, 0.7), color(1.0, 1.0, 0.984)),
            &[Light::new(
                Vec3::new(0.0, CHUNK_SIZE as f32, 0.0),
                color(1.0, 0.576, 0.184),
            )],
            Texture::from_bytes(renderer, include_bytes!("../assets/textures/grid.png")).unwrap(),
        );
        let unlit_material = UnlitMaterial::new(
            renderer,
            &renderer
                .device
                .create_shader_module(include_wgsl!("../assets/shaders/unlit.wgsl")),
        );

        unlit_meshes.extend([MeshBuilder::default()
            .with_added([0.0, 0.0, 0.0], 0..6)
            .build(renderer)]);

        let perlin = Perlin::new(0);

        for x in -1..=1 {
            for y in -1..1 {
                for z in -1..=1 {
                    let mut chunk = Chunk::new();
                    for dx in 0..CHUNK_SIZE {
                        for dy in 0..CHUNK_SIZE {
                            for dz in 0..CHUNK_SIZE {
                                const FREQUENCY: f64 = 0.05;
                                let value = perlin.get([
                                    (dx as f64 + x as f64 * CHUNK_SIZE as f64) * FREQUENCY,
                                    (dy as f64 + y as f64 * CHUNK_SIZE as f64) * FREQUENCY,
                                    (dz as f64 + z as f64 * CHUNK_SIZE as f64) * FREQUENCY,
                                ]);
                                if value >= 0.2 {
                                    chunk.set([dx, dy, dz], Some(Tile {}));
                                }
                            }
                        }
                    }

                    meshes.push(chunk.mesh(
                        renderer,
                        [
                            x as f32 * CHUNK_SIZE as f32,
                            y as f32 * CHUNK_SIZE as f32,
                            z as f32 * CHUNK_SIZE as f32,
                        ],
                    ));
                }
            }
        }

        Box::new(Self {
            meshes,
            unlit_meshes,

            camera,

            bound: true,

            material,
            unlit_material,
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
        self.unlit_material.update_uniforms(frame.renderer);

        // Create Pass
        let mut pass = frame.pass(Color::BLACK);
        // Apply Materials To Pass
        self.material.apply(&mut pass);

        for mesh in &self.meshes {
            pass.render_mesh(mesh);
        }

        // Apply Unlit Material To Pass
        self.unlit_material.apply(&mut pass);

        for mesh in &self.unlit_meshes {
            pass.render_mesh(mesh);
        }
    }

    fn exit(&mut self) {
        log::info!("Exiting Game!");
    }
}
