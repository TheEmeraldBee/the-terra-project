use crate::{prelude::*, scene::SceneFn};
use std::sync::Arc;

pub mod app_handler;

pub mod frame;

pub mod util;

pub mod prelude {
    pub use super::frame::Frame;
    pub use super::App;
}

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
