use clap::{crate_authors, crate_version, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    arg_required_else_help = true,
    disable_help_subcommand = true,
    after_help = format!("Crafted By {} | {} | MIT", crate_authors!(), crate_version!())
)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommands {
    #[command(subcommand, about = "all configuration commands.")]
    Config(FaCommandConfig),

    #[command(subcommand, about = "all store commands.")]
    Store(FaCommandStore),

    #[command(about = "list all the stored credentials.")]
    List {
        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,
    },

    #[command(about = "add a new credential.")]
    Add {
        #[arg(index = 1, help = "a required username/email.")]
        user: String,

        #[arg(index = 2, help = "a required password.")]
        password: String,

        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(long, short, required = false, help = "an optional tag name.")]
        tag: Option<String>,

        #[arg(long, short = 'w', required = false, help = "an optional website.")]
        site: Option<String>,
    },

    #[command(about = "remove an existing credential.")]
    Remove {
        #[arg(index = 1, help = "a required username/email.")]
        user: String,

        #[arg(index = 2, help = "a required password.")]
        password: String,

        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,
    },

    #[command(about = "search through your store.")]
    Search {
        #[arg(index = 1, help = "a required search query.")]
        query: String,

        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(
            long,
            short,
            required = false,
            help = "an optional filter.",
            long_help = "an optional filter that is applied onto your search query. the format is <filter>/<filter_query> where <filter> can be either 'site' or 'tag' and <filter_query> would be your filter specific query."
        )]
        filter: Option<String>,
    },

    #[command(about = "initialize 'fa' and create a configuration.")]
    Init {
        #[arg(
            long,
            short,
            help = "a required gpg key-id/fingerprint.",
            long_help = "a required gpg keyid/fingerprint. You can follow https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key till 'Step 4' to get your gpg key-id/fingerprint."
        )]
        key_fingerprint: Option<String>,

        #[arg(long, short = 's', help = "a required store name.")]
        store: Option<String>,

        #[arg(long, short = 'p', help = "a required store path")]
        store_path: Option<String>,
    },

    #[command(about = "import your credentials from a csv file to a store.")]
    Import {
        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(
            long,
            short = 'c',
            help = "a required csv file path.",
            long_help = "a required csv file path. the only requirement is that the csv file must contain a 'username', 'password' field and an optional 'url' field. "
        )]
        csv_path: Option<String>,
    },

    #[command(about = "export your credentails to a csv file from a store.")]
    Export {
        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(
            long,
            short = 'c',
            help = "a required csv file path.",
            long_help = "a required csv file path. you can provide any name to the csv file. 'fa' will either overwrite or create one for you."
        )]
        csv_path: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandConfig {
    #[command(about = "view the current configuration utilized by 'fa'.")]
    View,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandStore {
    #[command(about = "list all the stores.")]
    List,

    #[command(about = "remove a store.")]
    Remove {
        #[arg(index = 1, help = "a required store name.")]
        store: String,
    },

    #[command(about = "add a new empty store.")]
    Add {
        #[arg(index = 1, help = "a required store name.")]
        store: String,
    },

    #[command(about = "mark a store as default store.")]
    Default {
        #[arg(index = 1, help = "a required store name.")]
        store: String,
    },
}
