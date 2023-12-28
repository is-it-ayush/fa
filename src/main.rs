#![allow(clippy::redundant_field_names)]
#![deny(missing_docs)]

//! The main function of the program. Here we create an instance
//! of 'fa', run it and also print the bubbled (recoverable) errors.

use fa::Fa;

mod cli;
mod config;
mod error;
mod fa;
mod gpg;
mod store;

fn main() -> Result<(), String> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut fa = Fa::new();
    fa.run().map_err(|e| e.to_string())
}
