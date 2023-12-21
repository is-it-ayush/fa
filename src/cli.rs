use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommands {
    #[command(subcommand, about = "the configuration related commands.")]
    Config(FaCommandConfig),

    #[command(subcommand, long_about = "Stores are a grou store realted commands.")]
    Store(FaCommandStore),

    #[command(about = "list credentials from default store")]
    List {
        #[arg(long, short, required = false, long_help = "the store name (string).")]
        store: Option<String>,
    },

    #[command(about = "add a credential to the default store")]
    Add {
        #[arg(
            index = 1,
            long_help = "the login (string). this could be a email, username or anything else that is a string."
        )]
        user: String,

        #[arg(index = 2, long_help = "the password (string). this is your password.")]
        password: String,

        #[arg(
            long,
            short,
            required = false,
            long_help = "the store name (string). this is an optional argument and 'fa' uses a default store if the store does not exist (the default store is managed by 'fa'). if the store does not exist, 'fa' will create one for you."
        )]
        store: Option<String>,
    },

    Search {
        #[arg(
            index = 1,
            long_help = "the search query (string). this is the '<user>' from 'fa add <user> <password>' but it doesn't have to be complete."
        )]
        query: String,

        #[arg(
            long,
            short,
            required = false,
            long_help = "the store name (string)."
        )]
        store: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandConfig {
    View {},
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandStore {
    List {
        #[arg(
            long,
            short,
            required = false,
            long_help = "the store name (string). this is an optional argument and 'fa' uses a default store if the store does not exist (the default store is managed by 'fa'). if the store does not exist, 'fa' will create one for you."
        )]
        store: Option<String>,
    },
}
