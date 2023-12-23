#![allow(clippy::redundant_field_names)]

use error::FaError;
use fa::Fa;

mod cli;
mod config;
mod error;
mod fa;
mod gpg;
mod store;

fn main() -> Result<(), FaError> {
    let mut fa = Fa::new();
    fa.run()?;
    Ok(())
}
