use pine::prelude::{Pine, WindowConfig};
use tracing_subscriber::EnvFilter;

fn main() {
    let log_filter = EnvFilter::try_new("pine=trace")
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Failed to create tracing filter");
    tracing_subscriber::fmt().with_env_filter(log_filter).init();

    Pine::app()
        .with_window(WindowConfig::default().with_resizable(false))
        .run();
}
