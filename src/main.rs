use std::error::Error;
use buffengine::engine::application::Application;
use buffengine::logger;
use logger::init_logging;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let app = Application {};
    app.run();

    Ok(())
}
