use pine::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    App::new()?.run();
    Ok(())
}
