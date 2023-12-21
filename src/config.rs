use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::{FaError, FaErrorCodes};

#[derive(Debug, Clone)]
pub struct Config {
    pub config_path: PathBuf,
    pub _inner: InnerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerConfig {
    pub store: InnerConfigStore,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerConfigStore {
    pub store_path: String,
    pub default_store: String,
}

impl InnerConfig {
    /// Read the configuration from the disk.
    pub fn read(config_path: &PathBuf) -> Result<InnerConfig, FaError> {

        match fs::metadata(&config_path) {
            Ok(_) => {
                // read from existing config
                let config_str = fs::read_to_string(config_path)?;
                let config = toml::from_str::<InnerConfig>(config_str.as_str())?;
                Ok(config)
            }
            Err(_) => {
                // generate new config & save it on the path
                let config = InnerConfig::generate()?;
                Ok(config)
            }
        }
    }

    /// Generate a new configuration.
    pub fn generate() -> Result<InnerConfig, FaError> {
        let config_path = get_config_path()?;
        let store_path = get_store_path(&config_path)?
            .to_str()
            .ok_or(FaError::new(
                FaErrorCodes::GENERIC,
                "Could not convert path to string.",
            ))?
            .to_string();
        let config = InnerConfig {
            store: InnerConfigStore {
                store_path: store_path,
                default_store: String::from("fa_store"),
            },
        };
        Ok(config)
    }
}

impl Config {
    /// Initialize a configuration instance with a path.
    pub fn new() -> Result<Self, FaError> {
        let path = get_config_path()?;
        let inner_config = InnerConfig::read(&path)?;

        // ensure store directory.
        let store_path = get_store_path(&path)?;
        fs::create_dir_all(&store_path)?;

        Ok(Config {
            config_path: path,
            _inner: inner_config,
        })
    }
}

/// Helper function to get the configuration_path.
pub fn get_config_path() -> Result<PathBuf, FaError> {
    // read $HOME
    let home_path = env::var("HOME").map_err(FaError::from)?;
    let config_path_str = format!("{}/.config/fa/fa.toml", home_path);
    // example path: /home/ayush/.config/fa/fa.toml
    Ok(Path::new(&config_path_str).to_path_buf())
}

/// Helper function to get the store path from configuration_path.
pub fn get_store_path(config_path: &PathBuf) -> Result<PathBuf, FaError> {
    let mut stores_path_buf = config_path.clone();
    stores_path_buf.pop();
    stores_path_buf.push("stores");

    // example path: /home/ayush/.config/fa/stores
    Ok(stores_path_buf)
}

#[test]
fn test_config_path() {
    let config_path = get_config_path();
    assert!(config_path.is_ok());

    let unwrapped_path = config_path.unwrap().to_str().unwrap().to_string();
    assert!(unwrapped_path.len() > 0);
    assert!(unwrapped_path.contains("/.config/fa/fa.toml"));
}

#[test]
fn test_store_path() {
    let config_path = get_config_path().unwrap();
    let store_path = get_store_path(&config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(store_path.len() > 0);
    assert!(store_path.contains("/.config/fa/stores"));
}
