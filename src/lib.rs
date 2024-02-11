mod app;
mod error;
mod rendering;
mod windowing;

pub mod prelude {
    pub use crate::{app::Pine, rendering::Color, windowing::WindowConfig};
}
