use serde::{Deserialize, Serialize};

use crate::{error::FaError, gpg::Gpg};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub site: Option<String>,
    pub tag: Option<String>,
    pub password: String,
}

pub type StoreData = HashMap<String, Vec<Credential>>;

#[derive(Debug, Clone)]
pub struct Store {
    pub name: String,
    pub path: String,
    pub data: StoreData,
}

impl Store {
    pub fn new(name: &String, store_path: PathBuf, fingerprint: &String) -> Result<Self, FaError> {
        if Self::check_if_exists(&store_path) {
            return Err(FaError::AlreadyPresent { path: store_path });
        };

        // ensure parent directory exists.
        fs::create_dir_all(store_path.parent().ok_or(FaError::UnexpectedNone)?)?;

        let mut store_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(&store_path)?;

        let encrypted_data = match Gpg::encrypt(fingerprint, &String::new()) {
            Ok(d) => d,
            Err(e) => {
                fs::remove_file(&store_path)?;
                return Err(e);
            }
        };
        store_file.write_all(&encrypted_data)?;
        let store_path_string = store_path
            .to_str()
            .ok_or(FaError::UnexpectedNone)?
            .to_string();
        Ok(Self {
            name: name.to_string(),
            path: store_path_string,
            data: HashMap::new(),
        })
    }

    pub fn load(name: &String, store_path: PathBuf, fingerprint: &String) -> Result<Self, FaError> {
        // check if store exists.
        if !Self::check_if_exists(&store_path) {
            return Err(FaError::NoStore { path: store_path });
        }

        // load store.
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&store_path)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        let data = Gpg::decrypt(fingerprint, file_contents)?;
        let store_data: StoreData = match data.is_empty() {
            true => HashMap::new(),
            false => data,
        };

        Ok(Store {
            name: name.to_owned(),
            path: store_path
                .to_str()
                .ok_or(FaError::UnexpectedNone)?
                .to_string(),
            data: store_data,
        })
    }

    pub fn save(&self, fingerprint: &String) -> Result<(), FaError> {
        let data_str = serde_json::to_string(&self.data)?;
        let mut store_file = File::options()
            .write(true)
            .create(true)
            .append(false)
            .open(&self.path)?;
        let encrypted_data = Gpg::encrypt(fingerprint, &data_str)?;
        store_file.write_all(&encrypted_data)?;
        Ok(())
    }

    pub fn get_file_path(store_name: &String, base_path: &String) -> Result<PathBuf, FaError> {
        let store_file_name = format!("{}.fa", &store_name);
        let mut store_path = Path::new(&base_path).to_path_buf();
        store_path.push(&store_file_name);
        Ok(store_path)
    }

    pub fn check_if_exists(store_path: &PathBuf) -> bool {
        fs::metadata(store_path).is_ok()
    }
}
