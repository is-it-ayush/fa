use std::{env::VarError, error::Error, fmt::Display, io, string::FromUtf8Error};

#[derive(Debug, PartialEq)]
pub enum FaErrorCodes {
    Generic,
    Internal,
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
                FaErrorCodes::Internal,
                "The environment variable was not present",
                value,
            ),
            VarError::NotUnicode(e) => FaError::new(
                FaErrorCodes::Internal,
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
            code: FaErrorCodes::Internal,
            reason: String::from("Could not serialize configuration."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<toml::de::Error> for FaError {
    fn from(value: toml::de::Error) -> Self {
        FaError {
            code: FaErrorCodes::Internal,
            reason: String::from("Could not deserialize configuration."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<io::Error> for FaError {
    fn from(value: io::Error) -> Self {
        FaError {
            code: FaErrorCodes::Internal,
            reason: String::from("An IO error occured."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<serde_json::Error> for FaError {
    fn from(value: serde_json::Error) -> Self {
        FaError {
            code: FaErrorCodes::Internal,
            reason: String::from("Could not transform values."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<dialoguer::Error> for FaError {
    fn from(value: dialoguer::Error) -> Self {
        FaError {
            code: FaErrorCodes::Internal,
            reason: String::from("Could not ask for input."),
            source: Some(Box::new(value)),
        }
    }
}

impl From<FromUtf8Error> for FaError {
    fn from(_value: FromUtf8Error) -> Self {
        FaError::new(FaErrorCodes::Generic, "Could not convert buffer to string.")
    }
}

#[test]
fn test_new_error() {
    let error = FaError::new(FaErrorCodes::Generic, "Test Error.");
    assert_eq!(FaErrorCodes::Generic, error.code);
    assert_eq!(String::from("Test Error."), error.reason);
}

#[test]
fn test_new_error_from_source() {
    let io_error = io::Error::new(io::ErrorKind::Other, "Internal Test Error.");
    let error = FaError::from_source(FaErrorCodes::Internal, "Test Error.", io_error);
    assert_eq!(FaErrorCodes::Internal, error.code);
    assert_eq!(String::from("Test Error."), error.reason);
    assert!(error.source().is_some());
}
