use winit::{
    event::{Event as WinitEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    window::WindowId,
};

use crate::renderer::Renderer;

pub struct EventProcessor;

impl EventProcessor {
    /// Processes a winit event.
    pub fn process_event(
        event: WinitEvent<()>,
        elwt: &EventLoopWindowTarget<()>,
        renderer: &mut Renderer,
    ) {
        match event {
            WinitEvent::WindowEvent { window_id, event } => {
                Self::process_window_event(event, window_id, elwt, renderer)
            }
            _ => {}
        }
    }

    /// Process a window event (winit).
    fn process_window_event(
        event: WindowEvent,
        _id: WindowId,
        elwt: &EventLoopWindowTarget<()>,
        renderer: &mut Renderer,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            WindowEvent::RedrawRequested => {
                // Rendering call should go here...
                renderer.render();
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            _ => {}
        }
    }
}
