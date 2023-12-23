use crate::{error::FaError, gpg::Gpg};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub type StoreData = HashMap<String, Vec<String>>;

#[derive(Debug, Clone)]
pub struct Store {
    pub name: String,
    pub path: String,
    pub data: StoreData,
}

impl Store {
    pub fn load(name: &String, base_path: &String, fingerprint: &String) -> Result<Self, FaError> {
        // get store path.
        let store_path = Self::get_file_path(name, base_path)?;
        let store_path_str = store_path.to_str().ok_or(FaError::new(
            crate::error::FaErrorCodes::Generic,
            "Could not get store path.",
        ))?;

        // load store contents (create if non-existent).
        let mut store_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&store_path)?;
        let mut store_file_contents = Vec::new();
        store_file.read_to_end(&mut store_file_contents)?;

        // decrypt
        let data = Gpg::decrypt(fingerprint, store_file_contents)?;

        let data: StoreData = match data.is_empty() {
            true => HashMap::new(),
            false => data,
        };

        Ok(Store {
            name: name.to_owned(),
            path: store_path_str.to_string(),
            data: data,
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

    pub fn check_if_exists(store_name: &String, base_path: &String) -> Result<bool, FaError> {
        // get store path.
        let store_path = Self::get_file_path(store_name, base_path)?;
        Ok(fs::metadata(store_path).is_ok())
    }
}
