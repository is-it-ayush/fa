use crate::{error::FaError, gpg::Gpg};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
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

        // decrypt
        let data = Gpg::decrypt(fingerprint, store_path.clone())?;
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

    pub fn check_if_exists(store_path: &PathBuf) -> Result<bool, FaError> {
        Ok(fs::metadata(store_path).is_ok())
    }
}
