use std::error::Error;

use crate::args::parse_args;
use system_monitor::print_once;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = parse_args();

    if !args.live && !args.log {
        print_once()?;
    }

    Ok(())
}
