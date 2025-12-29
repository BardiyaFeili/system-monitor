use std::error::Error;
use crate::run::run;

mod args;
mod run;

fn main() -> Result<(), Box<dyn Error>>{
    run()?;
    
    Ok(())
}
