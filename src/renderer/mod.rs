pub mod backend;

use backend::{wgpu::Wgpu, Backend};

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Renderer {
    windows: Vec<Window>,
    backend: Box<dyn Backend>,
}

impl Renderer {
    /// Constructs a renderer with a default backend.
    pub async fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Pine Engine")
            .with_inner_size(LogicalSize::new(500, 500))
            .build(event_loop)?;
        let backend = Box::new(Wgpu::new(&window).await);

        let windows = vec![window];

        let renderer = Self { backend, windows };
        Ok(renderer)
    }

    /// Render using the current backend.
    pub fn render(&mut self) {
        let window = self
            .windows
            .get(0)
            .expect("failed to find window when rendering");
        self.backend.render(window);
    }

    /// Resize using the current backend.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.backend.resize(new_size.to_logical(1.));
    }
}
