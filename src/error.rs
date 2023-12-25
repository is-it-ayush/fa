#[derive(Debug, thiserror::Error)]
pub enum FaError {
    /// new
    #[error("Could not find a key associated with the provided fingerprint. Are you sure the fingerprint/key id is correct and the ascociated key is present on the system?")]
    InvalidFingerprint { fingerprint: String },

    /// new
    #[error(
        "Could not find a onfiguration file at {:?}. Have you ran 'fa init'?",
        path
    )]
    NoConfiguration { path: std::path::PathBuf },

    /// new
    #[error("Could not find a store at {:?}", path)]
    NoStore { path: std::path::PathBuf },

    /// new
    #[error(
        "Attempted to create a new store but a store was already present at {:?}",
        path
    )]
    AlreadyPresent { path: std::path::PathBuf },

    /// new
    #[error("Could not encrypt data for the store.")]
    GPGEncryptionError,

    /// new
    #[error("Could not decrypt data for the store.")]
    GPGDecryptionError,

    // result --> result
    #[error("{}", source)]
    IOError {
        #[from]
        source: std::io::Error,
    },

    /// result --> result
    #[error(
        "The environment variable ${} returned the error \"{}\"",
        variable,
        source
    )]
    EnvironmentVariableError {
        variable: String,
        #[source]
        source: std::env::VarError,
    },

    /// option --> result
    #[error("A value was expected but recieved none instead.")]
    UnexpectedNone,

    /// result --> result
    #[error(
        "Could not convert a byte vector to a string. It returned \"{}\"",
        source
    )]
    ByteVectorToString {
        #[from]
        source: std::string::FromUtf8Error,
    },

    /// result --> result
    #[error("Could not serialize configuration file. It returned \"{}\"", source)]
    SerializeConfiguration {
        #[from]
        source: toml::ser::Error,
    },

    /// result --> result
    #[error("Could not deserialize configuration file. It returned \"{}\"", source)]
    DeserializeConfiguration {
        #[from]
        source: toml::de::Error,
    },

    /// result --> result
    #[error("Could not serialize or deserialize store. It returned \"{}\"", source)]
    StoreDeOrSerialization {
        #[from]
        source: serde_json::Error,
    },

    /// result ---> result
    #[error("Could not take an input. It returned \"{}\"", source)]
    PromptError {
        #[from]
        source: dialoguer::Error,
    },
}
