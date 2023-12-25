#![allow(clippy::redundant_field_names)]

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
