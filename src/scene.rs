use winit::{event::WindowEvent, window::Window};

use crate::{render::Renderer, window::Frame};

pub type SceneFn = &'static dyn Fn(&Window, &Renderer) -> Box<dyn Scene>;

pub trait Scene {
    fn update(&mut self, frame: Frame) -> SceneEvent;
    fn event(&mut self, _event: &WindowEvent) {}
}

#[derive(Default)]
pub enum SceneEvent {
    ChangeScene(SceneFn),
    SetScene(Box<dyn Scene>),
    #[default]
    Empty,
}
