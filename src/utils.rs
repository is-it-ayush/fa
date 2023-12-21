use std::{ffi::OsStr};

use crate::error::{FaError, FaErrorCodes};

pub fn osstr_to_str(str: &OsStr) -> Result<&str, FaError> {
    str.to_str().ok_or(FaError::new(
        FaErrorCodes::GENERIC,
        "Could not convert os string to string.",
    ))
}
