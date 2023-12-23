use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommands {
    #[command(subcommand)]
    Config(FaCommandConfig),

    #[command(subcommand)]
    Store(FaCommandStore),

    #[command()]
    List {
        #[arg(long, short, required = false)]
        store: Option<String>,
    },

    #[command()]
    Add {
        #[arg(index = 1)]
        user: String,

        #[arg(index = 2)]
        password: String,

        #[arg(long, short, required = false)]
        store: Option<String>,
    },

    #[command()]
    Search {
        #[arg(index = 1)]
        query: String,

        #[arg(long, short, required = false)]
        store: Option<String>,
    },

    #[command()]
    Init {
        #[arg(long, short)]
        key_fingerprint: Option<String>,

        #[arg(long, short = 'n')]
        store: Option<String>,

        #[arg(long, short = 'p')]
        store_path: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandConfig {
    #[command()]
    View,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandStore {
    #[command()]
    List,
}
