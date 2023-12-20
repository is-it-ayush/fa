use std::{env::VarError, error::Error, fmt::Display, io};

#[derive(Debug, PartialEq)]
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

    pub fn from_source<E>(code: FaErrorCodes, reason: &str, error: E) -> Self
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

impl From<VarError> for FaError {
    fn from(value: VarError) -> Self {
        match &value {
            VarError::NotPresent => FaError::from_source(
                FaErrorCodes::INTERNAL,
                "The environment variable was not present",
                value,
            ),
            VarError::NotUnicode(e) => FaError::new(
                FaErrorCodes::INTERNAL,
                format!(
                    "The environment variable value is not unicode: {}",
                    e.to_str().unwrap()
                )
                .as_str(),
            ),
        }
    }
}

impl From<toml::ser::Error> for FaError {
    fn from(value: toml::ser::Error) -> Self {
        FaError {
            code: FaErrorCodes::INTERNAL,
            reason: String::from("Could not serialize configuration."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<toml::de::Error> for FaError {
    fn from(value: toml::de::Error) -> Self {
        FaError {
            code: FaErrorCodes::INTERNAL,
            reason: String::from("Could not deserialize configuration."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<io::Error> for FaError {
    fn from(value: io::Error) -> Self {
        FaError {
            code: FaErrorCodes::INTERNAL,
            reason: String::from("An IO error occured."),
            source: Some(Box::new(value)),
        }
    }
}

#[test]
fn test_new_error() {
    let error = FaError::new(FaErrorCodes::GENERIC, "Test Error.");
    assert_eq!(FaErrorCodes::GENERIC, error.code);
    assert_eq!(String::from("Test Error."), error.reason);
}

#[test]
fn test_new_error_from_source() {
    let io_error = io::Error::new(io::ErrorKind::Other, "Internal Test Error.");
    let error = FaError::from_source(FaErrorCodes::INTERNAL, "Test Error.", io_error);
    assert_eq!(FaErrorCodes::INTERNAL, error.code);
    assert_eq!(String::from("Test Error."), error.reason);
    assert!(error.source().is_some());
}
