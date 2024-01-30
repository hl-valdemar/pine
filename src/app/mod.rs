mod config;
pub mod events;

use config::AppConfig;
use events::event_processor::EventProcessor;

use crate::renderer::Renderer;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

pub struct App {
    config: AppConfig,
    renderer: Renderer,
    event_loop: EventLoop<()>,
}

impl App {
    /// Constructs a new application context.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::default();
        let event_loop = EventLoop::new()?;
        let renderer = pollster::block_on(Renderer::new(&event_loop))?;

        let app = App {
            config,
            renderer,
            event_loop,
        };
        Ok(app)
    }

    /// Explicitly sets the control flow for the application.
    ///
    /// Control flow options include:
    /// * Polling (`ControlFlow::Poll`)
    /// * Waiting (`ControlFlow::Wait`)
    /// * Waiting until (`ControlFlow::WaitUntil(Instant)`)
    pub fn with_control_flow(&mut self, control_flow: ControlFlow) {
        self.config.control_flow = control_flow;
    }

    /// Runs the application.
    pub fn run(mut self) {
        // Initialize the application
        self.init();

        let _ = self.event_loop.run(move |event, elwt| {
            EventProcessor::process_event(event, elwt, &mut self.renderer);
        });
    }

    /// Initializes the application based on the application config.
    fn init(&mut self) {
        self.event_loop.set_control_flow(self.config.control_flow);
    }
}
