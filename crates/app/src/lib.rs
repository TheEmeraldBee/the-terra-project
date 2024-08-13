use std::sync::Arc;

use input::Input;
use render::renderer::Renderer;
use scene::{Scene, SceneFn, SceneState};
use time::Time;
use winit::window::Window;

pub mod events;
pub mod frame;
pub mod input;
pub mod scene;
pub mod time;

pub mod window_extension;

pub mod handler;

pub struct App<'a> {
    renderer: Option<Renderer<'a>>,
    window: Option<Arc<Window>>,
    scene: SceneState,
    input: Input,
    time: Time,
}

impl<'a> App<'a> {
    pub fn new(scene: SceneFn) -> Self {
        Self {
            renderer: None,
            window: None,
            scene: SceneState::Unloaded(scene),
            input: Input::default(),
            time: Time::default(),
        }
    }

    pub fn new_preload(scene: Box<dyn Scene>) -> Self {
        Self {
            renderer: None,
            window: None,
            scene: SceneState::Loaded(scene),
            input: Input::default(),
            time: Time::default(),
        }
    }
}
