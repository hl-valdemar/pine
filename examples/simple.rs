use pine::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()?.run();
    Ok(())
}
