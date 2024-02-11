#[derive(Debug, Clone, Copy)]
/// A custom Color type.
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}
