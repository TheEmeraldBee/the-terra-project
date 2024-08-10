use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        // Ignore this as windows only have one id.
        _window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match self.window.as_mut() {
            Some(t) => t,
            None => {
                log::warn!(
                    r#"Event Loop Ran Without Window, returning without doing anything.

                    If you see this many times, and the window never loads, consider reporting a bug."#
                );
                return;
            }
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => window.request_redraw(),
            _ => {}
        }
    }
}
