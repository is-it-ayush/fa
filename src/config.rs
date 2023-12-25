use crate::error::FaError;
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
        let store_path_buf = Path::new(&store_base_path).absolutize()?;
        let store_path: String = store_path_buf
            .to_str()
            .ok_or(FaError::UnexpectedNone)?
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
        let mut configuration_path = get_base_directory()?;
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
        let file_path_string = format!("{}/config.toml", get_base_directory()?);
        let file_path = Path::new(&file_path_string);

        match fs::metadata(file_path) {
            Ok(_) => {
                let file_content = fs::read_to_string(file_path)?;
                let inner_config = toml::from_str::<InnerConfig>(&file_content)?;

                Ok(Config {
                    config_file_path: file_path_string,
                    _inner: inner_config,
                })
            }
            Err(_) => Err(FaError::NoConfiguration {
                path: file_path.to_path_buf(),
            }),
        }
    }
}

pub fn get_base_directory() -> Result<String, FaError> {
    let home_variable = String::from("HOME");
    let home_path =
        std::env::var(&home_variable).map_err(|e| FaError::EnvironmentVariableError {
            variable: home_variable,
            source: e,
        })?;
    Ok(format!("{}/.config/fa", home_path))
}
