use std::io::stdout;
use std::time::SystemTime;
use fern::{Dispatch, InitError};
use LevelFilter::Debug;
use log::LevelFilter;
use humantime::format_rfc3339_seconds;

pub fn init_logging() -> Result<(), InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(Debug)
        .chain(stdout())
        .apply()?;
    Ok(())
}