#![allow(non_shorthand_field_patterns)]

use error::{BandhitaError, BandhitaErrorCodes};

mod error;

fn main() -> Result<(), BandhitaError> {
    Err(BandhitaError::new(
        BandhitaErrorCodes::GENERIC,
        "HEEEEEEEEEEEEEEEEEEEEEEEEEEE :3",
    ))
}
