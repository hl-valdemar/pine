use pine::prelude::{Color, Pine, WindowConfig};
use tracing_subscriber::EnvFilter;

fn main() {
    let log_filter = EnvFilter::try_new("pine=trace")
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Failed to create tracing filter");
    tracing_subscriber::fmt().with_env_filter(log_filter).init();

    Pine::app()
        .with_window(WindowConfig::default().with_clear_color(Color::RED))
        .with_window(
            WindowConfig::default()
                .with_title("Second window!")
                .with_clear_color(Color::BLUE),
        )
        .with_window(
            WindowConfig::default()
                .with_title("Non-resizable window 😯")
                .with_resizable(false)
                .with_clear_color(Color::GREEN),
        )
        .run();
}
