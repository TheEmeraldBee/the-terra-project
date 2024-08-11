use crate::prelude::*;

pub type SceneFn = &'static dyn Fn(&Window, &Renderer) -> Box<dyn Scene>;

pub trait Scene {
    fn update(&mut self, frame: Frame) -> SceneEvent;
    fn event(&mut self, _event: &WindowEvent, _renderer: &Renderer, _window: &Window) {}
}

#[derive(Default)]
pub enum SceneEvent {
    ChangeScene(SceneFn),
    SetScene(Box<dyn Scene>),
    #[default]
    Empty,
}

pub enum SceneState {
    Unloaded(SceneFn),
    Loaded(Box<dyn Scene>),
}

impl SceneState {
    pub fn loaded(&mut self) -> &mut Box<dyn Scene> {
        match self {
            Self::Loaded(t) => t,
            _ => panic!("Scene is not loaded. This is a bug, please report!"),
        }
    }
}

pub mod prelude {
    pub use super::{Scene, SceneEvent, SceneState};
}
