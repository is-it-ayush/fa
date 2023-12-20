use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug)]
pub enum FaCommands {
    #[command(subcommand, about = "the configuration related commands.")]
    Config(FaCommandConfig),

    #[command(subcommand, about = "the id and password store realted commands.")]
    Store(FaCommandStore),

    #[command(about = "list credentials from default store")]
    List,
}

#[derive(Subcommand, Debug)]
pub enum FaCommandConfig {
    View {

    }
}

#[derive(Subcommand, Debug)]
pub enum FaCommandStore {}
