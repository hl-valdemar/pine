#[derive(Debug)]
/// Data relevent for the rendering step.
///
/// Produced in the preparation step.
pub struct FrameData<'surface> {
    pub clear_color: wgpu::Color,
    pub surface: wgpu::Surface<'surface>,
}

#[derive(Debug)]
/// A builder for the FrameData that allows for gradually setting the different values of the frame
/// data.
pub struct FrameDataBuilder<'surface> {
    pub clear_color: Option<wgpu::Color>,
    pub surface: Option<wgpu::Surface<'surface>>,
}

impl<'b> Default for FrameDataBuilder<'b> {
    fn default() -> Self {
        Self {
            clear_color: Some(wgpu::Color::BLACK),
            surface: None,
        }
    }
}

impl<'b> FrameDataBuilder<'b> {
    /// Sets the surface to render.
    pub fn with_surface(mut self, surface: wgpu::Surface<'b>) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Sets the clear color to render with.
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    /// Constructs the actual FrameData used in the render step.
    pub fn build(self) -> FrameData<'b> {
        FrameData {
            clear_color: self.clear_color.unwrap(),
            surface: self.surface.expect("No surface found in frame builder"),
        }
    }
}
