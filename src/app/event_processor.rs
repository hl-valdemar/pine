use winit::{
    event::{Event as WinitEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
};

pub struct EventProcessor;

impl EventProcessor {
    /// Processes a winit event.
    pub fn process_event(event: WinitEvent<()>, elwt: &EventLoopWindowTarget<()>) {
        match event {
            WinitEvent::WindowEvent {
                window_id: _window_id,
                event,
            } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
