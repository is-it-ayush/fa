use crate::{
    cli::{FaCli, FaCommandConfig, FaCommands},
    config::Config,
    error::FaError,
};
use clap::Parser;

#[derive(Debug)]
pub struct Fa {
    pub config: Config,
    cli: FaCli,
}

impl Fa {
    pub fn new() -> Result<Self, FaError> {
        let fa_cli = FaCli::parse();
        let config = Config::new()?;
        Ok(Fa {
            cli: fa_cli,
            config: config,
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
                let path = &self.config.path.to_str().ok_or(FaError::new(
                    crate::error::FaErrorCodes::GENERIC,
                    "Failed to convert path from pathbuf to string.",
                ))?;
                let store_path = &self.config._inner.store.store_path;
                let store = &self.config._inner.store.default_store;

                println!("==== fa config. ====");
                println!("path: {}", path);
                println!("");
                println!("==== store. ====");
                println!("path: {}", store_path);
                println!("default: {}", store);

                Ok(())
            }
        }
    }

    fn command_list(&self) -> Result<(), FaError> {
        Ok(())
    }
}
