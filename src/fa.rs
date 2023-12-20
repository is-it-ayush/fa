use crate::{
    cli::{FaCli, FaCommandConfig, FaCommands},
    config::Config,
    error::FaError,
    store::Store,
};
use clap::Parser;

#[derive(Debug)]
pub struct Fa {
    pub config: Config,
    pub default_store: Store,
    cli: FaCli,
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

    pub fn execute(&self) -> Result<(), FaError> {
        match &self.cli.command {
            Some(FaCommands::Config(fc)) => self.command_config(fc),
            Some(FaCommands::List) => self.command_list(),
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
        Ok(())
    }
}
