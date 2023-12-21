use crate::{
    cli::{FaCli, FaCommandConfig, FaCommands},
    config::Config,
    error::FaError,
    store::Store,
};
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Fa {
    pub config: Config,
    pub default_store: Store,
    pub cli: FaCli,
}

impl Fa {
    /// Create a new application instance.
    pub fn new() -> Result<Self, FaError> {
        let fa_cli = FaCli::parse();
        let config = Config::new()?;
        let default_store = Store::load(
            &config._inner.store.default_store,
            &config._inner.store.store_path,
        )?;
        Ok(Fa {
            cli: fa_cli,
            config: config,
            default_store: default_store,
        })
    }

    /// Execute and match on the entrypoint command.
    pub fn execute(&mut self) -> Result<(), FaError> {
        let cloned_command = &self.cli.command.clone();
        match cloned_command {
            Some(FaCommands::Config(fc)) => self.command_config(&fc),
            Some(FaCommands::List { store }) => self.command_list(&store),
            Some(FaCommands::Add {
                user,
                password,
                store,
            }) => self.command_add(&user, &password, &store),
            Some(FaCommands::Store(_fs)) => todo!(),
            Some(FaCommands::Search { query, store }) => self.command_search(&query, &store),
            None => Ok(()),
        }
    }

    /// Grab the store to work on.
    pub fn get_store(&self, passed_store: &Option<String>) -> Result<Store, FaError> {
        let store: Store = match passed_store {
            Some(store_name) => {
                match Store::check_if_exists(store_name, &self.config._inner.store.store_path)? {
                    true => Store::load(store_name, &self.config._inner.store.store_path)?,
                    false => {
                        return Err(FaError {
                            code: crate::error::FaErrorCodes::GENERIC,
                            reason: String::from("The store does not exist."),
                            source: None,
                        })
                    }
                }
            }
            None => self.default_store.clone(),
        };

        Ok(store)
    }

    /// Match on the `fa config`.
    fn command_config(&self, command_config: &FaCommandConfig) -> Result<(), FaError> {
        match command_config {
            FaCommandConfig::View {} => {
                let store_path = &self.config._inner.store.store_path;
                let store = &self.config._inner.store.default_store;

                println!("store_path = {}", store_path);
                println!("default_store = {}", store);

                Ok(())
            }
        }
    }

    /// Execute `fa list`
    fn command_list(&self, passed_store: &Option<String>) -> Result<(), FaError> {
        let store = self.get_store(&passed_store)?;

        println!("==== {} store ====", &store.name);
        if store.data.is_empty() {
            println!("the store is currently empty. add a login & a password to view it here!");
            println!("fa add <login> <password> --");
        } else {
            for (key, group) in store.data.iter() {
                for val in group.iter() {
                    println!("{key} : {val}");
                }
            }
        }

        Ok(())
    }

    /// Execute `fa add`
    fn command_add(
        &mut self,
        user: &String,
        password: &String,
        passed_store: &Option<String>,
    ) -> Result<(), FaError> {
        let mut store: Store = match passed_store {
            Some(store_name) => Store::load(store_name, &self.config._inner.store.store_path)?,
            None => self.default_store.clone(),
        };

        store
            .data
            .entry(user.to_owned())
            .or_insert_with(Vec::new)
            .push(password.to_owned());
        store.save()?;

        println!("{} was successfully added to {} ", &user, &store.name);
        Ok(())
    }

    fn command_search(
        &mut self,
        passed_query: &String,
        passed_store: &Option<String>,
    ) -> Result<(), FaError> {
        let store = self.get_store(&passed_store)?;
        let query = passed_query.to_lowercase();

        for (key, group) in store.data.iter() {
            if key.to_lowercase().starts_with(&query) {
                for val in group.iter() {
                    println!("{key} : {val}");
                }
            }
        }

        Ok(())
    }
}
