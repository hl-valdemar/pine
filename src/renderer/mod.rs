pub mod backend;

use backend::{wgpu::Wgpu, Backend};

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Renderer {
    window: Window,
    backend: Box<dyn Backend>,
}

impl Renderer {
    /// Constructs a renderer with a default backend.
    pub async fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Test run")
            .with_inner_size(LogicalSize::new(500, 500))
            .build(event_loop)?;
        let backend = Box::new(Wgpu::new(&window).await);

        let renderer = Self { backend, window };
        Ok(renderer)
    }

    /// Render using the current backend.
    pub fn render(&mut self) {
        self.backend.render(&self.window);
    }

    /// Resize using the current backend.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.backend.resize(new_size.to_logical(1.));
    }
}
