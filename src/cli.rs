use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommands {
    #[command(subcommand, about = "All configuration related commands go here.")]
    Config(FaCommandConfig),

    #[command(subcommand, about = "All store related commands go here.")]
    Store(FaCommandStore),

    #[command(about = "List credentials from the 'default' store.")]
    List {
        #[arg(
            long,
            short,
            required = false,
            help = "Optional Store name.",
            long_help = "Optional store name. If the store does not already exist, 'fa.' will throw an error."
        )]
        store: Option<String>,
    },

    #[command(about = "Add a credential to the 'default' store.")]
    Add {
        #[arg(
            index = 1,
            help = "The username/email/login. It is required.",
            long_help = "The 'user' could be an email, username or literally anything else. This is generally an email or username. Is is requied."
        )]
        user: String,

        #[arg(
            index = 2,
            help = "The password. It is required.",
            long_help = "This is the password string. It is required."
        )]
        password: String,

        #[arg(
            long,
            short,
            required = false,
            help = "Optional store name.",
            long_help = "Optional store name. If the store is provided and does not already exists. 'fa.' creates a store and then adds the credetianls to the store."
        )]
        store: Option<String>,
    },

    #[command(about = "Search the store with a query.")]
    Search {
        #[arg(
            index = 1,
            help = "A query string. It is required.",
            long_help = "A required search query string. This is usually the '<user>' from 'fa add <user> <password>' but it doesn't have to be complete i.e. you could search through the store with a partially complete '<user>'."
        )]
        query: String,

        #[arg(
            long,
            short,
            required = false,
            help = "Optional store name.",
            long_help = "Optional store name. If the store does not already exist, 'fa.' will throw an error."
        )]
        store: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandConfig {
    #[command(about = "View the current 'fa' configuration.")]
    View,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandStore {
    #[command(about = "List all the stores.")]
    List,
}
