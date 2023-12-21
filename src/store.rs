use crate::error::FaError;
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
    pub path: PathBuf,
    pub data: StoreData,
}

impl Store {
    pub fn load(name: &String, base_path: &String) -> Result<Self, FaError> {
        // get store path.
        let store_path = Self::get_file_path(&name, &base_path)?;

        // load store contents (create if non-existent).
        let mut store_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&store_path)?;
        let mut store_file_contents = String::new();
        store_file.read_to_string(&mut store_file_contents)?;

        // transform data.
        let data: StoreData = match store_file_contents.is_empty() {
            true => HashMap::new(),
            false => serde_json::from_str::<StoreData>(&store_file_contents)?,
        };

        Ok(Store {
            name: name.to_owned(),
            path: store_path,
            data: data,
        })
    }

    pub fn save(&self) -> Result<(), FaError> {
        let data_str = serde_json::to_string(&self.data)?;
        let mut store_file = File::options()
            .write(true)
            .create(true)
            .append(false)
            .open(&self.path)?;
        store_file.write_all(&data_str.as_bytes())?;
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
        let store_path = Self::get_file_path(&store_name, &base_path)?;
        Ok(fs::metadata(&store_path).is_ok())
    }
}
