use std::sync::Arc;
use wgpu::{include_wgsl, RenderPipeline};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::render::Renderer;

#[derive(Default)]
pub struct App<'a> {
    renderer: Option<Renderer<'a>>,
    window: Option<Arc<Window>>,
    pipelines: Vec<RenderPipeline>,
}

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

        self.renderer = match Renderer::new(self.window.clone().unwrap(), (1920, 1080)) {
            Ok(r) => Some(r),
            Err(e) => {
                log::error!("Creating renderer failed with error: {e}");
                event_loop.exit();
                return;
            }
        };

        self.pipelines.push(
            self.renderer
                .as_ref()
                .unwrap()
                .render_pipeline_descriptor(include_wgsl!("../assets/shaders/basic.wgsl")),
        )
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
                log::warn!(
                    r#"Event Loop Ran Without Renderer, this is fatal, and should be reported as a bug."#
                );
                event_loop.exit();
                return;
            }
        };

        match event {
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
                renderer.frame(|view, encoder| {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    rpass.set_pipeline(&self.pipelines[0]);
                    rpass.draw(0..3, 0..1);
                });
                window.request_redraw();
            }
            _ => {}
        }
    }
}
