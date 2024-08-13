use render::camera::Camera;

pub enum AppEvent {
    ApplyCamera(Camera),
}

pub struct Events {
    pub(crate) events: Vec<AppEvent>,
}

impl Events {
    pub(crate) fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn register(&mut self, event: AppEvent) {
        self.events.push(event);
    }
}
