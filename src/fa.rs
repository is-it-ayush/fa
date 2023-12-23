use crate::{
    cli::{FaCli, FaCommandConfig, FaCommandStore, FaCommands},
    config::Config,
    error::{FaError, FaErrorCodes},
    gpg::Gpg,
    store::Store,
};
use clap::Parser;
use dialoguer::Input;
use std::{ffi::OsStr, fs, path::Path};

#[derive(Debug, Clone)]
pub struct Fa {
    cli: FaCli,
}

#[derive(Debug, Clone)]
pub struct FaApplicationState {
    configuration: Config,
}

impl Fa {
    pub fn new() -> Self {
        Self {
            cli: FaCli::parse(),
        }
    }

    pub fn run(&mut self) -> Result<(), FaError> {
        let cloned_command = &self.cli.command.clone();

        // create/get configuration.
        let config: Config;
        if let Some(FaCommands::Init {
            key_fingerprint,
            store,
            store_path,
        }) = cloned_command
        {
            config = self.command_init(key_fingerprint, store, store_path)?;
        } else {
            config = Config::load_from_disk()?;
        }

        // initialize state.
        let state = FaApplicationState {
            configuration: config,
        };

        match cloned_command {
            // command group
            Some(FaCommands::Config(fc)) => self.command_group_config(fc, &state),
            Some(FaCommands::Store(fs)) => self.command_group_store(fs, &state),

            // command
            Some(FaCommands::List { store }) => self.command_list(store, &state),
            Some(FaCommands::Add {
                user,
                password,
                store,
            }) => self.command_add(user, password, store, &state),
            Some(FaCommands::Search { query, store }) => self.command_search(query, store, &state),

            // do nothing
            Some(FaCommands::Init { .. }) => Ok(()), // handled specially
            None => Ok(()),
        }
    }

    pub fn get_or_create_store(
        &self,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<Store, FaError> {
        let store_name = match passed_store {
            Some(sn) => sn,
            None => &state.configuration._inner.store.default_store,
        };
        Store::load(store_name, &state.configuration._inner.store.base_path)
    }

    pub fn get_store(
        &self,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<Store, FaError> {
        let store_name = match passed_store {
            Some(sn) => sn,
            None => &state.configuration._inner.store.default_store,
        };
        if !Store::check_if_exists(store_name, &state.configuration._inner.store.base_path)? {
            Err(FaError {
                code: FaErrorCodes::Generic,
                reason: String::from("The store does not exist"),
                source: None,
            })
        } else {
            Store::load(store_name, &state.configuration._inner.store.base_path)
        }
    }

    /// Command Groups

    fn command_group_config(
        &self,
        command_config: &FaCommandConfig,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        match command_config {
            FaCommandConfig::View => {
                let store_path = &state.configuration._inner.store.base_path;
                let store = &state.configuration._inner.store.default_store;

                println!("fa: store_path = {}", store_path);
                println!("fa: default_store = {}", store);

                Ok(())
            }
        }
    }

    fn command_group_store(
        &self,
        command_store: &FaCommandStore,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        match command_store {
            FaCommandStore::List => {
                let store_path = &state.configuration._inner.store.base_path;
                for entry in (fs::read_dir(store_path)?).flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            // get file name
                            let file_name_osstr = entry.file_name();
                            let file_name = &file_name_osstr.to_str().ok_or(FaError::new(
                                FaErrorCodes::Generic,
                                "Could not convert file name to string.",
                            ))?;

                            let extension = Path::new(&file_name)
                                .extension()
                                .and_then(OsStr::to_str)
                                .ok_or(FaError::new(
                                    FaErrorCodes::Generic,
                                    "Could not extract extension of the filename.",
                                ))?;
                            if extension == "fa" {
                                println!("fa: {}", &file_name);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Command

    fn command_list(
        &self,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let store = self.get_store(passed_store, state)?;

        println!("fa: using '{}' store", &store.name);
        if store.data.is_empty() {
            println!("fa: the store is currently empty. add a login & a password to view it here!");
            println!("fa: fa add <login> <password> --");
        } else {
            for (key, group) in store.data.iter() {
                for val in group.iter() {
                    println!("fa: {key} : {val}");
                }
            }
        }

        Ok(())
    }

    fn command_add(
        &mut self,
        user: &String,
        password: &String,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let mut store: Store = self.get_or_create_store(passed_store, state)?;

        store
            .data
            .entry(user.to_owned())
            .or_insert_with(Vec::new)
            .push(password.to_owned());
        store.save()?;

        println!("fa: {} was successfully added to {} ", &user, &store.name);
        Ok(())
    }

    fn command_search(
        &mut self,
        passed_query: &str,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let store = self.get_store(passed_store, state)?;
        let query = passed_query.to_lowercase();

        for (key, group) in store.data.iter() {
            if key.to_lowercase().starts_with(&query) {
                for val in group.iter() {
                    println!("fa: {key} : {val}");
                }
            }
        }

        Ok(())
    }

    pub fn command_init(
        &self,
        passed_key_fingerprint: &Option<String>,
        passed_store: &Option<String>,
        passed_store_path: &Option<String>,
    ) -> Result<Config, FaError> {
        // get fingerprint
        let fingerprint: String = match passed_key_fingerprint {
            Some(p_fgp) => {
                // verify
                if !Gpg::check_if_fingerprint_exists(p_fgp)? {
                    Gpg::fingerprint_prompt_until_true_or_term()?
                } else {
                    p_fgp.to_owned()
                }
            }
            None => {
                // prompt
                Gpg::fingerprint_prompt_until_true_or_term()?
            }
        };

        let store_name: String = match passed_store {
            Some(p_store) => p_store.to_owned(),
            None => Input::new()
                .with_prompt("Enter a default store name")
                .interact_text()?,
        };

        let store_path: String = match passed_store_path {
            Some(p_store_path) => p_store_path.to_owned(),
            None => Input::new()
                .with_prompt("Enter a path for all your stores")
                .interact_text()?,
        };

        let config = Config::new(store_path, store_name, fingerprint)?; //
        Ok(config)
    }
}
