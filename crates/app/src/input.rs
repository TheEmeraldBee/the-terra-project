use glam::*;
use wgpu::naga::FastHashSet;
use winit::{
    event::{ElementState, MouseScrollDelta, RawKeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Input {
    just_pressed: FastHashSet<KeyCode>,
    pressed: FastHashSet<KeyCode>,
    just_released: FastHashSet<KeyCode>,

    mouse_just_pressed: FastHashSet<u32>,
    mouse_pressed: FastHashSet<u32>,
    mouse_just_released: FastHashSet<u32>,

    mouse_pos: Vec2,
    mouse_delta: Vec2,

    scroll_delta: Vec2,
}

impl Input {
    pub fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();

        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();

        self.mouse_delta = Vec2::ZERO;

        self.scroll_delta = Vec2::ZERO;
    }

    pub fn event(&mut self, event: RawKeyEvent) {
        match event.state {
            winit::event::ElementState::Pressed => {
                let code = match event.physical_key {
                    PhysicalKey::Unidentified(_) => {
                        log::info!("Key with unknown value just pressed, consider changing your keybinds for this key. \nThis key will now be processed as ESC.");
                        KeyCode::Escape
                    }
                    PhysicalKey::Code(c) => c,
                };

                self.pressed.insert(code);
                self.just_pressed.insert(code);
            }
            winit::event::ElementState::Released => {
                let code = match event.physical_key {
                    PhysicalKey::Unidentified(_) => KeyCode::Escape,
                    PhysicalKey::Code(c) => c,
                };

                self.pressed.remove(&code);
                self.just_released.insert(code);
            }
        }
    }

    pub fn scroll_event(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => self.scroll_delta += Vec2::new(x, y),
            _ => {
                log::warn!("Pixel based scrolling is not supported.")
            }
        }
    }

    pub fn mouse_positioned(&mut self, position: (f32, f32)) {
        self.mouse_pos = position.into();
    }

    pub fn mouse_moved(&mut self, delta: (f64, f64)) {
        self.mouse_delta += Vec2::from((delta.0 as f32, delta.1 as f32));
    }

    pub fn mouse_event(&mut self, button: u32, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.mouse_just_pressed.insert(button);
                self.mouse_pressed.insert(button);
            }
            ElementState::Released => {
                self.mouse_pressed.remove(&button);
                self.mouse_just_released.insert(button);
            }
        }
    }

    pub fn pressed(&self, code: KeyCode) -> bool {
        self.pressed.contains(&code)
    }
    pub fn just_pressed(&self, code: KeyCode) -> bool {
        self.just_pressed.contains(&code)
    }
    pub fn just_released(&self, code: KeyCode) -> bool {
        self.just_released.contains(&code)
    }

    pub fn mouse_pressed(&self, button: u32) -> bool {
        self.mouse_pressed.contains(&button)
    }
    pub fn mouse_just_pressed(&self, button: u32) -> bool {
        self.mouse_just_pressed.contains(&button)
    }
    pub fn mouse_just_released(&self, button: u32) -> bool {
        self.mouse_just_released.contains(&button)
    }

    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn scroll(&self) -> Vec2 {
        self.scroll_delta
    }

    pub fn key_vector(&self, up: KeyCode, left: KeyCode, down: KeyCode, right: KeyCode) -> Vec2 {
        vec2(self.key_value(right, left), self.key_value(up, down))
    }

    pub fn key_value(&self, pos: KeyCode, neg: KeyCode) -> f32 {
        let mut res = 0.0;
        if self.pressed(pos) {
            res += 1.0;
        }
        if self.pressed(neg) {
            res -= 1.0;
        }
        res
    }
}
