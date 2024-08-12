use crate::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, RawKeyEvent},
    window::WindowId,
};

use std::sync::Arc;

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(Window::default_attributes()) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                log::error!("Creating window failed with error: {e}");
                event_loop.exit();
                return;
            }
        };
        self.window = Some(window.clone());

        self.renderer = match Renderer::build(self.window.clone().unwrap(), (1920, 1080)) {
            Ok(r) => Some(r),
            Err(e) => {
                log::error!("Creating renderer failed with error: {e}");
                event_loop.exit();
                return;
            }
        };

        match self.scene {
            SceneState::Unloaded(f) => {
                self.scene = SceneState::Loaded(f(&window, self.renderer.as_ref().unwrap()))
            }
            SceneState::Loaded(_) => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        // Ignore this as windows only have one id.
        _window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match self.window.as_ref() {
            Some(t) => t.as_ref(),
            None => {
                log::warn!(
                    r#"Event Loop Ran Without Window, returning without doing anything.

                    If you see this many times, and the window never loads, consider reporting a bug."#
                );
                return;
            }
        };
        let renderer = match self.renderer.as_mut() {
            Some(t) => t,
            None => {
                log::error!(
                    r#"Event Loop Ran Without Renderer, this is fatal, and should be reported as a bug."#
                );
                event_loop.exit();
                return;
            }
        };

        let scene = match self.scene {
            SceneState::Unloaded(f) => {
                self.scene = SceneState::Loaded(f(window, renderer));
                self.scene.loaded()
            }
            _ => self.scene.loaded(),
        };

        match &event {
            WindowEvent::Resized(size) => {
                // Tell the renderer to resize
                renderer.resize((size.width, size.height));

                // For Mac, request a redraw of the frame.
                window.request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.time.update();

                let frame = Frame {
                    renderer,
                    window,
                    input: &self.input,
                    time: &self.time,
                };

                match scene.update(frame) {
                    SceneEvent::ChangeScene(f) => self.scene = SceneState::Unloaded(f),
                    SceneEvent::SetScene(s) => self.scene = SceneState::Loaded(s),
                    SceneEvent::Empty => {}
                };
                self.input.update();
                window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => self
                .input
                .mouse_positioned((position.x as f32, position.y as f32)),
            WindowEvent::KeyboardInput { event, .. } => {
                self.input.event(RawKeyEvent {
                    physical_key: event.physical_key,
                    state: event.state,
                });
            }
            _ => {}
        }

        // Send event to scene in case it wants custom processing.
        match &mut self.scene {
            SceneState::Unloaded(_) => {}
            SceneState::Loaded(l) => l.event(&event, renderer, window),
        }
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::Key(k) => self.input.event(k),
            DeviceEvent::MouseMotion { delta } => self.input.mouse_moved(delta),
            DeviceEvent::Button { button, state } => self.input.mouse_event(button, state),
            DeviceEvent::MouseWheel { delta } => self.input.scroll_event(delta),
            _ => {}
        }
    }
}