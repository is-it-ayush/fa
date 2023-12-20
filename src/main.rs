use error::FaError;
use fa::Fa;

mod cli;
mod config;
mod error;
mod fa;

fn main() -> Result<(), FaError> {
    let fa = Fa::new()?;
    fa.execute()?;
    Ok(())
}
