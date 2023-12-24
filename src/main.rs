#![allow(clippy::redundant_field_names)]

use std::process::ExitCode;

use fa::Fa;

mod cli;
mod config;
mod error;
mod fa;
mod gpg;
mod store;

fn main() -> ExitCode {
    let mut fa = Fa::new();
    match fa.run() {
        Ok(_) => {
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
