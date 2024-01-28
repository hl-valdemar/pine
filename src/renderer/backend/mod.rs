pub mod wgpu;

pub trait Backend {}

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
