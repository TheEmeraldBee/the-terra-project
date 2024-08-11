use crate::prelude::*;

pub struct Frame<'a, 'r> {
    pub renderer: &'a Renderer<'r>,
    pub window: &'a Window,
    pub input: &'a Input,
    pub time: &'a Time,
}
