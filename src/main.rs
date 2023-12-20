use error::FaError;
use fa::Fa;

mod cli;
mod config;
mod error;
mod fa;
mod store;

fn main() -> Result<(), FaError> {
    let fa = Fa::new()?;
    fa.execute()?;
    Ok(())
}
