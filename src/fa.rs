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
            None => Ok(()),
        }
    }

    fn command_config(&self, command_config: &FaCommandConfig) -> Result<(), FaError> {
        match command_config {
            FaCommandConfig::View {} => {
                let store_path = &self.config._inner.store.store_path;
                let store = &self.config._inner.store.default_store;

                println!("==== store. ====");
                println!("path = {}", store_path);
                println!("default = {}", store);

                Ok(())
            }
        }
    }

    fn command_list(&self, passed_store: &Option<String>) -> Result<(), FaError> {
        let store: Store = match passed_store {
            Some(store_name) => Store::load(store_name, &self.config._inner.store.store_path)?,
            None => self.default_store.clone(),
        };

        println!("==== {} store ====", &store.name);
        if store.data.is_empty() {
            println!("the store is currently empty. add a login & a password to view it here!");
            println!("fa add <login> <password>");
        } else {
            for (key, val) in store.data.iter() {
                println!("{key} : {val}");
            }
        }

        Ok(())
    }

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

        store.data.insert(user.to_owned(), password.to_owned());
        store.save()?;
        println!("{} was successfully added to {} ", &user, &store.name);
        Ok(())
    }
}
