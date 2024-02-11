use winit::{
    event::{Event as WinitEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
};

use crate::{
    error::PineError,
    windowing::{Window, WindowConfig},
};

/// Holds the relevant items for the Pine engine.
pub struct Pine {
    windows: Vec<Window>,
}

/// The Pine configuration.
pub struct PineConfig {
    window_configs: Vec<WindowConfig>,
}

impl Pine {
    /// Returns a new Pine config.
    ///
    /// Allows for nicer ergonomics.
    ///
    /// # Example
    ///
    /// ```
    /// Pine::app().with_window(WindowConfig::default()).run();
    /// ```
    pub fn app() -> PineConfig {
        PineConfig::new()
    }

    /// Constructs a new Pine instance.
    pub fn new(windows: Vec<Window>) -> Self {
        Self { windows }
    }

    /// Spins up the pine engine.
    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let result = event_loop
            .run(|event, elwt| match event {
                WinitEvent::WindowEvent { window_id, event } => match event {
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.logical_key == Key::Named(NamedKey::Space) {
                            tracing::info!("Space key pressed!");
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        if let Some(window) = self
                            .windows
                            .iter()
                            .find(|window| window.handle.id() == window_id)
                        {
                            let frame_data = window.renderer.prepare(&window);
                            if let Some(frame_data) = frame_data.ok() {
                                window.renderer.render(&frame_data);
                            }
                        } else {
                            tracing::warn!(
                                "Redraw requested for window {:?} but no such window was found",
                                window_id
                            );
                        }
                    }
                    WindowEvent::Resized(new_size) => {
                        tracing::info!("Window {:?} resized to {:?}", window_id, new_size);
                        if let Some(window) = self
                            .windows
                            .iter_mut()
                            .find(|window| window.handle.id() == window_id)
                        {
                            window.renderer.resize(new_size);
                        }
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        inner_size_writer: _inner_size_writer,
                    } => {
                        tracing::info!("Scale factor changed to {}", scale_factor);
                    }
                    WindowEvent::CloseRequested => {
                        tracing::info!("Window close requested for window {:?}", window_id);
                        if let Some(i) = self
                            .windows
                            .iter()
                            .position(|window| window.handle.id() == window_id)
                        {
                            self.windows.remove(i);
                            tracing::info!("Window {:?} closed", window_id);
                        }

                        if self.windows.is_empty() {
                            tracing::info!("No more windows. Shutting down...");
                            elwt.exit();
                        }
                    }
                    _ => {}
                },
                _ => {}
            })
            .map_err(|err| PineError::EventLoopError(err));

        match result {
            Ok(_) => (),
            Err(err) => tracing::error!("Pine error: {:?}", err),
        }
    }
}

impl PineConfig {
    /// Constructs a new Pine config.
    pub fn new() -> Self {
        PineConfig {
            window_configs: vec![],
        }
    }

    /// Appends a window to the config.
    ///
    /// NB: windows will be built at the same time as the Pine instance.
    pub fn with_window(&mut self, config: WindowConfig) -> &mut Self {
        self.window_configs.push(config);
        self
    }

    /// Constructs a Pine instance from the config.
    pub fn build(&mut self, event_loop: &EventLoop<()>) -> Pine {
        let windows = self
            .window_configs
            .iter()
            .map(|config| config.build(&event_loop).expect("Failed to build window"))
            .collect();

        Pine::new(windows)
    }

    /// Shortcut to spin up Pine from config.
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("Failed to initialize event loop");
        self.build(&event_loop).run(event_loop);
    }
}
