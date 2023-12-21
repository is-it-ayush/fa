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
            Some(FaCommands::List) => self.command_list(),
            Some(FaCommands::Add { user, password }) => self.command_add(&user, &password),
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

    fn command_list(&self) -> Result<(), FaError> {
        println!("==== default store ====");

        if self.default_store.data.is_empty() {
            println!("the store is currently empty. add a login & a password to view it here!");
            println!("fa add <login> <password>");
        } else {
            for (key, val) in self.default_store.data.iter() {
                println!("{key} : {val}");
            }
        }

        Ok(())
    }

    fn command_add(&mut self, user: &String, password: &String) -> Result<(), FaError> {
        self.default_store
            .data
            .insert(user.to_owned(), password.to_owned());
        self.default_store.save()?;
        Ok(())
    }
}
