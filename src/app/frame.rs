use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Frame<'a, 'r> {
    pub renderer: &'a Renderer<'r>,
    pub window: &'a Window,
    pub input: &'a Input,
    pub time: &'a Time,
}
