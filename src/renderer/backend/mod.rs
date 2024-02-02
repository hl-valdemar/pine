pub mod wgpu;

use winit::{dpi::LogicalSize, window::Window};

pub trait Backend {
    fn resize(&mut self, new_size: LogicalSize<u32>);

    fn render(&mut self, window: &Window);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
