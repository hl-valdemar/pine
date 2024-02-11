use crate::{
    error::PineError,
    rendering::{color::Color, Renderer},
};

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoopWindowTarget,
    window::{Window as WinitWindow, WindowBuilder},
};

#[derive(Debug)]
/// The Window struct holds a handle to the winit Window as well as the Renderer attached to said
/// window (among other things).
pub struct Window {
    pub handle: WinitWindow,
    pub renderer: Renderer,
    pub clear_color: Color,
}

#[derive(Debug, Clone)]
/// The Window config defines customizable options for building a window.
pub struct WindowConfig {
    title: String,
    width: Option<u32>,
    height: Option<u32>,
    clear_color: Option<Color>,
    resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Pine".to_string(),
            width: Some(500),
            height: Some(500),
            clear_color: Some(Color::BLACK),
            resizable: true,
        }
    }
}

impl WindowConfig {
    /// Sets the title of the window.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets whether the window is resizable.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets the clear color of the window.
    pub fn with_clear_color(mut self, color: Color) -> Self {
        self.clear_color = Some(color);
        self
    }

    /// Constructs an actual Pine window from the config.
    pub fn build(&self, elwt: &EventLoopWindowTarget<()>) -> Result<Window, PineError> {
        let mut builder = WindowBuilder::new()
            .with_title(self.title.as_str())
            .with_resizable(self.resizable);

        if let Some((width, height)) = self.width.zip(self.height) {
            builder = builder.with_inner_size(LogicalSize::new(width, height));
        }

        let clear_color = if let Some(color) = self.clear_color {
            color
        } else {
            Color::BLACK
        };

        let handle = builder.build(elwt).map_err(|err| {
            tracing::error!("Failed to build window");
            return PineError::OsError(err);
        })?;
        let renderer =
            pollster::block_on(Renderer::new(&handle)).expect("Failed to construct renderer");

        let window = Window {
            handle,
            renderer,
            clear_color,
        };
        Ok(window)
    }
}
