#[derive(Debug, thiserror::Error)]
pub enum FaError {
    /// new
    #[error("configuration file is not present at: {:?}", path)]
    NoConfiguration { path: std::path::PathBuf },

    /// new
    #[error("store file is not present at: {:?}", path)]
    NoStore { path: std::path::PathBuf },

    /// new
    #[error("could not encrypt data.")]
    GPGEncryptionError,

    /// new
    #[error("could not decrypt file: {:?}", path)]
    GPGDecryptionError { path: std::path::PathBuf },

    // result --> result
    #[error("input output error: {}", source)]
    IOError {
        #[from]
        source: std::io::Error,
    },

    /// result --> result
    #[error("${}: {}", variable, source)]
    EnvironmentVariableError {
        variable: String,
        #[source]
        source: std::env::VarError,
    },

    /// option --> result
    #[error("expected a value but recieved none instead.")]
    UnexpectedNone,

    /// result --> result
    #[error("could not convert byte vector to string: {}", source)]
    ByteVectorToString {
        #[from]
        source: std::string::FromUtf8Error,
    },

    /// result --> result
    #[error("could not serialize configuration: {}", source)]
    SerializeConfiguration {
        #[from]
        source: toml::ser::Error,
    },

    /// result --> result
    #[error("could not deserialize configuration: {}", source)]
    DeserializeConfiguration {
        #[from]
        source: toml::de::Error,
    },

    /// result --> result
    #[error("serialization or deserialization error: {}", source)]
    JsonError {
        #[from]
        source: serde_json::Error,
    },

    /// result ---> result
    #[error("prompt error: {}", source)]
    PromptError {
        #[from]
        source: dialoguer::Error,
    },
}
