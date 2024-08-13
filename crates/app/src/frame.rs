use winit::window::Window;

use crate::{events::Events, input::Input, time::Time};

pub struct UpdateFrame<'a> {
    pub input: &'a Input,
    pub time: &'a Time,
    pub window: &'a Window,

    pub events: Events,
}
