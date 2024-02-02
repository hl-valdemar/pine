use winit::{
    event::{Event as WinitEvent, MouseScrollDelta, WindowEvent},
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
                tracing::info!("Close requested: closing...");
                elwt.exit();
            }
            WindowEvent::RedrawRequested => {
                renderer.render();
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _inner_size_writer,
            } => {
                tracing::info!("Scale factor changed: scale factor = {}", scale_factor);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_change = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
                };
                tracing::info!("Mouse wheel scrolled: delta = {}", zoom_change);
            }
            _ => {}
        }
    }
}
