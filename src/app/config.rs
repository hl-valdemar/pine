use winit::event_loop::ControlFlow;

#[derive(Debug)]
/// Holds configuration data for the application.
pub struct AppConfig {
    pub control_flow: ControlFlow,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            control_flow: ControlFlow::Poll,
        }
    }
}
