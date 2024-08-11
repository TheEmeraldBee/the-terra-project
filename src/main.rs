use winit::event_loop::EventLoop;

pub mod window;
use crate::window::App;

pub mod render;

fn main() {
    // Initialize the logger.
    env_logger::init();

    // Create an event loop.
    let event_loop: EventLoop<()> = match EventLoop::builder().build() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Event Loop Failed To Initialize with error:\n{e}");

            // Exit the App Because Window Is Important
            return;
        }
    };

    match event_loop.run_app(&mut App::default()) {
        Ok(_) => {}
        Err(_) => {}
    }
}
