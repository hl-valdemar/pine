pub mod backend;

use backend::{opengl::OpenGL, wgpu::Wgpu, Backend};

use std::sync::Arc;

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Renderer {
    window: Arc<Window>,
    backend: Box<dyn Backend>,
}

impl Renderer {
    /// Constructs a renderer with a default backend.
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Test run")
            .with_inner_size(LogicalSize::new(500, 500))
            .build(event_loop)?;
        let window = Arc::new(window);
        let backend = Box::new(OpenGL {});

        let renderer = Self { backend, window };
        Ok(renderer)
    }
}
