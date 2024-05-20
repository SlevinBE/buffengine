use std::error::Error;
use buffengine::engine::application::Application;
use buffengine::logger;

fn main() -> Result<(), Box<dyn Error>> {
    logger::init_logging()?;

    let app = Application {};
    app.run();

    Ok(())
}
