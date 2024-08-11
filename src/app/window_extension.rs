use winit::window::Window;

pub trait WindowExtensions {
    fn lock_cursor(&self, lock: bool);
}

impl WindowExtensions for Window {
    fn lock_cursor(&self, lock: bool) {
        if lock {
            self.set_cursor_grab(winit::window::CursorGrabMode::Locked)
                .unwrap();
            self.set_cursor_visible(false);
        } else {
            self.set_cursor_grab(winit::window::CursorGrabMode::None)
                .unwrap();
            self.set_cursor_visible(true);
        }
    }
}
