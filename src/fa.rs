use crate::{
    cli::{FaCli, FaCommandConfig, FaCommandStore, FaCommands},
    config::Config,
    error::FaError,
    gpg::Gpg,
    store::{Credential, Store},
};
use clap::Parser;
use dialoguer::Input;
use path_absolutize::Absolutize;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs::{self, File}, path::Path,
};

/// The struct is only used for import and export.
#[derive(Debug, Deserialize)]
struct ParsedCredential {
    username: String,
    password: String,
    url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Fa {
    cli: FaCli,
}
use console::{style, Emoji};

pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
pub static KEY: Emoji<'_, '_> = Emoji("🔑 ", "");
pub static MOAI: Emoji<'_, '_> = Emoji("🗿 ", "");
pub static ROAD: Emoji<'_, '_> = Emoji("🛣️ ", "");

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
        let mut state = FaApplicationState {
            configuration: config,
        };

        match cloned_command {
            // command group
            Some(FaCommands::Config(fc)) => self.command_group_config(fc, &state),
            Some(FaCommands::Store(fs)) => self.command_group_store(fs, &mut state),

            // command
            Some(FaCommands::List { store }) => self.command_list(store, &state),
            Some(FaCommands::Add {
                user,
                password,
                store,
                site,
                tag,
            }) => self.command_add(user, password, store, site, tag, &state),
            Some(FaCommands::Remove {
                user,
                password,
                store,
            }) => self.command_remove(user, password, store, &state),
            Some(FaCommands::Search {
                query,
                store,
                filter,
            }) => self.command_search(query, store, filter, &state),
            Some(FaCommands::Import { store, csv_path }) => {
                self.command_import(store, csv_path, &state)
            }
            Some(FaCommands::Export { store, csv_path }) => {
                self.command_export(store, csv_path, &state)
            }

            // do nothing
            Some(FaCommands::Init { .. }) => Ok(()),
            None => Ok(()),
        }
    }

    pub fn get_store(
        &self,
        passed_store: &Option<String>,
        state: &FaApplicationState,
        create_new: bool,
    ) -> Result<Store, FaError> {
        let store_name = match passed_store {
            Some(sn) => sn,
            None => &state.configuration._inner.store.default_store,
        };
        let store_file_path =
            Store::get_file_path(store_name, &state.configuration._inner.store.base_path)?;

        match Store::check_if_exists(&store_file_path) {
            true => Store::load(
                store_name,
                store_file_path,
                &state.configuration._inner.security.gpg_fingerprint,
            ),
            false => {
                if create_new {
                    Store::new(
                        store_name,
                        store_file_path,
                        &state.configuration._inner.security.gpg_fingerprint,
                    )
                } else {
                    Err(FaError::NoStore {
                        path: store_file_path,
                    })
                }
            }
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
                let configuration_path = &state.configuration.config_file_path;
                let store_path = &state.configuration._inner.store.base_path;
                let store = &state.configuration._inner.store.default_store;
                let fingerprint = &state.configuration._inner.security.gpg_fingerprint;

                let fa_header = style("fa").bold().dim();
                println!(
                    "{} | Located Configuration At '{}'",
                    fa_header,
                    style(configuration_path).bold()
                );
                println!("{} | store.path: {}", fa_header, store_path);
                println!("{} | store.default_store: {}", fa_header, store);
                println!("{} | security.fingerprint: {} ", fa_header, fingerprint);

                Ok(())
            }
        }
    }

    fn command_group_store(
        &self,
        command_store: &FaCommandStore,
        state: &mut FaApplicationState,
    ) -> Result<(), FaError> {
        match command_store {
            FaCommandStore::List => {
                let store_path = &state.configuration._inner.store.base_path;
                println!(
                    "{} | Using store directory '{}'",
                    style("fa").bold().dim(),
                    style(&store_path).bold().bright()
                );
                if fs::read_dir(store_path)?.count() == 0 {
                    println!("{} | There are no stores yet.", style("fa").bold().dim());
                } else {
                    for entry in fs::read_dir(store_path)?.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                let file_name_osstring = entry.file_name();
                                let file_name_with_extension =
                                    file_name_osstring.to_str().ok_or(FaError::UnexpectedNone)?;
                                let (file_name, _) = file_name_with_extension
                                    .rsplit_once('.')
                                    .ok_or(FaError::UnexpectedNone)?;
                                let extension = Path::new(file_name_with_extension)
                                    .extension()
                                    .and_then(OsStr::to_str)
                                    .ok_or(FaError::UnexpectedNone)?;

                                if extension == "fa" {
                                    println!("{} | {}", style("fa").bold().dim(), file_name);
                                }
                            }
                        }
                    }
                }
            }
            FaCommandStore::Remove { store } => {
                let store_path =
                    Store::get_file_path(store, &state.configuration._inner.store.base_path)?;
                if !Store::check_if_exists(&store_path) {
                    return Err(FaError::NoStore { path: store_path });
                } else {
                    // prompt for password before decrypting.
                    Store::load(
                        store,
                        store_path.clone(),
                        &state.configuration._inner.security.gpg_fingerprint,
                    )?;
                    fs::remove_file(&store_path)?;
                    println!(
                        "{} | {} removed {} store.",
                        style("fa").bold().dim(),
                        style("Successfully").bold().green(),
                        style(store).bright()
                    );
                }
            }
            FaCommandStore::Add { store } => {
                let store_path =
                    Store::get_file_path(store, &state.configuration._inner.store.base_path)?;
                Store::new(
                    store,
                    store_path,
                    &state.configuration._inner.security.gpg_fingerprint,
                )?;
                println!(
                    "{} | {} added {} store.",
                    style("fa").bold().dim(),
                    style("Successfully").bold().green(),
                    style(store).bright()
                );
            }
            FaCommandStore::Default { store } => {
                // check if exists.
                let store_path =
                    Store::get_file_path(store, &state.configuration._inner.store.base_path)?;
                match Store::check_if_exists(&store_path) {
                    true => {
                        let config = Config::new(
                            state.configuration._inner.store.base_path.clone(),
                            store.clone(),
                            state.configuration._inner.security.gpg_fingerprint.clone(),
                        )?;
                        state.configuration = config;
                        println!(
                            "{} | {} is now your {} store.",
                            style("fa").bold().dim(),
                            style(store).bold().green(),
                            style("default").bold().bright(),
                        );
                    }
                    false => {
                        println!(
                            "{} | The store {} does not exist.",
                            style("fa").bold().dim(),
                            style(store).bold().red()
                        );
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
        let store = self.get_store(passed_store, state, false)?;

        println!(
            "{} | Using store '{}'",
            style("fa").bold().dim(),
            style(&store.name).bold().bright()
        );
        if store.data.is_empty() {
            println!(
                "{} | The store is currently empty.",
                style("fa").bold().dim()
            );
            println!(
                "{} | try 'fa add <login> <password>'",
                style("fa").bold().dim()
            );
        } else {
            for cred in store.data.iter() {
                let site = match &cred.site {
                    Some(s) => s,
                    None => "-",
                };

                let tag = match &cred.tag {
                    Some(t) => t,
                    None => "-",
                };
                println!(
                    "{} | {} | {} | {} | {}",
                    style("fa").bold().dim(),
                    cred.user,
                    cred.password,
                    site,
                    tag
                );
            }
        }

        Ok(())
    }

    fn command_add(
        &mut self,
        user: &str,
        password: &str,
        passed_store: &Option<String>,
        passed_site: &Option<String>,
        passed_tag: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let mut store: Store = self.get_store(passed_store, state, true)?;

        // check if exists.
        if store
            .data
            .iter()
            .any(|s| s.user.eq(user) && s.password.eq(password))
        {
            return Err(FaError::CredentialsAlreadyExists);
        }

        // does not exist, create.
        store.data.push(Credential {
            password: String::from(password),
            user: String::from(user),
            tag: passed_tag.to_owned(),
            site: passed_site.to_owned(),
        });

        // save store.
        store.save(&state.configuration._inner.security.gpg_fingerprint)?;

        // tell user.
        println!(
            "{} | You've {} added '{}' login to {} store.",
            style("fa").bold().dim(),
            style("successfully").green(),
            style(&user).bold().bright(),
            style(&store.name).bold().bright()
        );
        Ok(())
    }

    fn command_remove(
        &mut self,
        user: &str,
        password: &str,
        passed_store: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let mut store: Store = self.get_store(passed_store, state, true)?;
        let credential_index = store
            .data
            .iter()
            .position(|cred| cred.user == user && cred.password == password);

        if let Some(index) = credential_index {
            store.data.remove(index);
            println!(
                "{} | You've {} removed '{}' login to {} store.",
                style("fa").bold().dim(),
                style("successfully").green(),
                style(&user).bold().bright(),
                style(&store.name).bold().bright()
            );
            // save store.
            store.save(&state.configuration._inner.security.gpg_fingerprint)?;
        }

        Ok(())
    }

    fn command_search(
        &mut self,
        passed_query: &str,
        passed_store: &Option<String>,
        passed_filter: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let store = self.get_store(passed_store, state, false)?;
        let query = passed_query.to_lowercase();

        if passed_filter.is_none() {
            println!(
                "{} | Searching '{}' on {} store...",
                style("fa").bold().dim(),
                style(&query).bold().green().bright(),
                style(store.name).bold().bright()
            );
            for cred in store.data.iter() {
                if cred.user.to_lowercase().starts_with(&query) {
                    let site = match &cred.site {
                        Some(s) => s,
                        None => "-",
                    };
                    let tag = match &cred.tag {
                        Some(t) => t,
                        None => "-",
                    };
                    println!(
                        "{} | {} | {} | {} | {}",
                        style("fa").bold().dim(),
                        cred.user,
                        cred.password,
                        site,
                        tag
                    );
                }
            }
        } else {
            let _allowed_filters = Vec::from(["tag", "site"]);
            let _filter = passed_filter
                .as_ref()
                .ok_or(FaError::UnexpectedNone)?
                .as_str();

            // verify <filter>/<filter_query> pattern.
            let starts_with_filter = _allowed_filters
                .iter()
                .any(|filter| _filter.starts_with(format!("{}/", &filter).as_str()));
            if !_filter.contains('/') || !starts_with_filter {
                return Err(FaError::UnexpectedFilter);
            }

            // split
            let splits = _filter.splitn(2, '/').collect::<Vec<_>>();
            let (filter, filter_query) = (splits[0], splits[1]);

            println!(
                "{} | Searching '{}' on {} store with filter '{}' and filter value '{}'...",
                style("fa").bold().dim(),
                style(&query).bold().green().bright(),
                style(store.name).bold().bright(),
                style(&filter).bold().red().bright(),
                style(&filter_query).bold().green().bright()
            );

            for cred in store.data.iter() {
                if cred.user.to_lowercase().starts_with(&query) {
                    let site = match &cred.site {
                        Some(s) => s,
                        None => "-",
                    };
                    let tag = match &cred.tag {
                        Some(t) => t,
                        None => "-",
                    };
                    match filter {
                        "site" => {
                            if site.starts_with(filter_query) {
                                println!(
                                    "{} | {} | {} | {} | {}",
                                    style("fa").bold().dim(),
                                    cred.user,
                                    cred.password,
                                    site,
                                    tag
                                );
                            }
                        }
                        "tag" => {
                            if tag.starts_with(filter_query) {
                                println!(
                                    "{} | {} | {} | {} | {}",
                                    style("fa").bold().dim(),
                                    cred.user,
                                    cred.password,
                                    site,
                                    tag
                                );
                            }
                        }
                        _ => {
                            println!("You know, this code should've never been executed. I think my code sucks.");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn command_import(
        &mut self,
        passed_store: &Option<String>,
        passed_csv_path: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let mut store = self.get_store(passed_store, state, true)?;
        let csv_file_path_string = match passed_csv_path {
            Some(passed_path) => passed_path.to_owned(),
            None => Input::<String>::new()
                .with_prompt(format!(
                    "{} | Where is the \".csv\" file that you'd like to try import from?",
                    ROAD
                ))
                .interact_text()?,
        };
        let csv_file_path = Path::new(&csv_file_path_string).absolutize()?;
        let mut csv_reader = csv::Reader::from_path(&csv_file_path)?;
        let mut cred_count = 0;

        for record_result in csv_reader.deserialize() {
            let record: ParsedCredential = record_result?;

            // skip if exists.
            if store
                .data
                .iter()
                .any(|s| s.user.eq(&record.username) && s.password.eq(&record.password))
            {
                println!(
                    "{} | Skipping '{}'. It as already present.",
                    style("fa").bold().dim(),
                    style(&record.username).bold().red().bright(),
                );
                continue;
            }

            // does not exist, add.
            store.data.push(Credential {
                password: String::from(&record.password),
                user: String::from(&record.username),
                tag: None,
                site: record.url,
            });
            cred_count += 1;

            // tell user.
            println!(
                "{} | You've {} added '{}' login to {} store.",
                style("fa").bold().dim(),
                style("successfully").green(),
                style(&record.username).bold().bright(),
                style(&store.name).bold().bright()
            );
        }

        // save store.
        store.save(&state.configuration._inner.security.gpg_fingerprint)?;

        println!(
            "{} | {} imported {} credentials from csv file {} to {} store.",
            style("fa").bold().dim(),
            style("Successfully").green(),
            style(cred_count).bold().bright(),
            style(&csv_file_path.to_str().ok_or(FaError::UnexpectedNone)?)
                .bold()
                .bright(),
            style(&store.name).bold().bright()
        );

        Ok(())
    }

    pub fn command_export(
        &mut self,
        passed_store: &Option<String>,
        passed_csv_path: &Option<String>,
        state: &FaApplicationState,
    ) -> Result<(), FaError> {
        let store = self.get_store(passed_store, state, false)?;
        let csv_file_path_string = match passed_csv_path {
            Some(passed_path) => passed_path.to_owned(),
            None => Input::<String>::new()
                .with_prompt(format!(
                    "{} | Could you provide the path to the \".csv\" file that you would want to export your credentials to?",
                    ROAD
                ))
                .interact_text()?,
        };
        let csv_file_path = Path::new(&csv_file_path_string).absolutize()?;
        let csv_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(&csv_file_path)?;
        let mut csv_writer = csv::Writer::from_writer(csv_file);
        let mut cred_count = 0;

        csv_writer.write_record(["username", "password", "url"])?;
        for record in store.data.iter() {
            let url = match record.site.clone() {
                Some(u) => u,
                None => String::new(),
            };
            csv_writer.write_record([&record.user, &record.password, &url])?;
            cred_count += 1;
            println!(
                "{} | You've {} exported '{}' login to {} store.",
                style("fa").bold().dim(),
                style("successfully").green(),
                style(&record.user).bold().bright(),
                style(&store.name).bold().bright()
            );
        }
        csv_writer.flush()?;

        println!(
            "{} | {} exported {} credentials from {} store to {}.",
            style("fa").bold().dim(),
            style("Successfully").green(),
            style(cred_count).bold().bright(),
            style(&store.name).bold().bright(),
            style(&csv_file_path.to_str().ok_or(FaError::UnexpectedNone)?)
                .bold()
                .bright()
        );

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

        let mut store_name: String = match passed_store {
            Some(p_store) => p_store.to_owned(),
            None => {
                let prompt_str = format!(
                    "{} | {}What would you call your default credential store?",
                    style("[2/3]").bold().dim(),
                    MOAI
                );
                Input::new().with_prompt(prompt_str).interact_text()?
            }
        };
        store_name = store_name.replace(' ', "_");

        let store_path: String = match passed_store_path {
            Some(p_store_path) => p_store_path.to_owned(),
            None => {
                let prompt_str = format!(
                    "{} | {}Where would all the stores be located at?",
                    style("[3/3]").bold().dim(),
                    ROAD
                );
                Input::new().with_prompt(prompt_str).interact_text()?
            }
        };

        let config = Config::new(store_path, store_name.clone(), fingerprint)?; //
        println!("{} | {}Successfully {} a config. You can now run '{}' to add a new credential to {} store.",
                 style("fa").bold().dim(),
                 SPARKLE,
                 style("generated").bold().green(),
                 style("fa add <login> <password>").bright(),
                 style(store_name).bright());
        Ok(config)
    }
}
