use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FaErrorCodes {
    GENERIC,
    INTERNAL,
}

#[derive(Debug)]
pub struct FaError {
    pub code: FaErrorCodes,
    pub reason: String,
    pub source: Option<Box<dyn Error>>,
}

impl Display for FaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for FaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl FaError {
    pub fn new(code: FaErrorCodes, reason: &str) -> Self {
        FaError {
            code: code,
            reason: reason.to_string(),
            source: None,
        }
    }

    pub fn from<E>(code: FaErrorCodes, reason: &str, error: E) -> Self
    where
        E: Error + 'static,
    {
        FaError {
            code: code,
            reason: reason.to_string(),
            source: Some(Box::new(error)),
        }
    }
}
