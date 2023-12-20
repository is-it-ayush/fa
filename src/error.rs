use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BandhitaErrorCodes {
    GENERIC,
    INTERNAL,
}

#[derive(Debug)]
pub struct BandhitaError {
    pub code: BandhitaErrorCodes,
    pub reason: String,
    pub source: Option<Box<dyn Error>>,
}

impl Display for BandhitaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for BandhitaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl BandhitaError {
    pub fn new(code: BandhitaErrorCodes, reason: &str) -> Self {
        BandhitaError {
            code: code,
            reason: reason.to_string(),
            source: None,
        }
    }

    pub fn from<E>(code: BandhitaErrorCodes, reason: &str, error: E) -> Self
    where
        E: Error + 'static,
    {
        BandhitaError {
            code: code,
            reason: reason.to_string(),
            source: Some(Box::new(error)),
        }
    }
}
