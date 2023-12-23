use crate::error::{FaError, FaErrorCodes};
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub config_file_path: String,
    pub _inner: InnerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerConfig {
    pub store: InnerConfigStore,
    pub security: InnerConfigSecurity,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerConfigStore {
    pub base_path: String,
    pub default_store: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerConfigSecurity {
    pub gpg_fingerprint: String,
}

impl Config {
    pub fn new(
        store_base_path: String,
        store_name: String,
        security_gpg_fingerprint: String,
    ) -> Result<Self, FaError> {
        let store_path = Path::new(&store_base_path)
            .absolutize()?
            .to_str()
            .ok_or(FaError::new(
                FaErrorCodes::Generic,
                format!("Could not get the absolute path from {}.", &store_base_path).as_str(),
            ))?
            .to_string();
        let inner_config = InnerConfig {
            store: InnerConfigStore {
                base_path: store_path,
                default_store: store_name,
            },
            security: InnerConfigSecurity {
                gpg_fingerprint: security_gpg_fingerprint,
            },
        };

        // ensure store directory exists.
        fs::create_dir_all(&store_base_path)?;

        // ensure config directory exists.
        let mut configuration_path = get_fa_from_home()?;
        fs::create_dir_all(&configuration_path)?;
        configuration_path = format!("{}/config.toml", configuration_path);

        // save to disk
        let config_str = toml::to_string(&inner_config)?;
        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .create(true)
            .truncate(true)
            .open(&configuration_path)?;
        config_file.write_all(config_str.as_bytes())?;

        Ok(Config {
            config_file_path: configuration_path,
            _inner: inner_config,
        })
    }

    pub fn load_from_disk() -> Result<Self, FaError> {
        let fa_dir = get_fa_from_home()?;
        let configuration_path = format!("{}/config.toml", fa_dir);

        match fs::metadata(&configuration_path) {
            Ok(_) => {
                // load and transform
                let config_file_content = fs::read_to_string(&configuration_path)?;
                let inner_config = toml::from_str::<InnerConfig>(&config_file_content)?;
                Ok(Config {
                    config_file_path: configuration_path,
                    _inner: inner_config,
                })
            }
            Err(_) => Err(FaError::new(
                FaErrorCodes::Generic,
                "Could not find a configuration file.",
            )),
        }
    }
}

pub fn get_fa_from_home() -> Result<String, FaError> {
    let home_path = std::env::var("HOME")?;
    Ok(format!("{}/.config/fa/", home_path))
}
