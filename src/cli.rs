use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct FaCli {
    #[command(subcommand)]
    pub command: Option<FaCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommands {
    #[command(subcommand, about = "all other configuration related commands.")]
    Config(FaCommandConfig),

    #[command(subcommand, about = "all other store related commands.")]
    Store(FaCommandStore),

    #[command(about = "list all the stored credentials.")]
    List {
        #[arg(long, short, required = false, help = "an optional store name")]
        store: Option<String>,
    },

    #[command(about = "add a new credential.")]
    Add {
        #[arg(
            index = 1,
            help = "a required login.",
            long_help = "this could be your email, username, login or any other identifier that goes along with your password."
        )]
        user: String,

        #[arg(index = 2, help = "a required password.")]
        password: String,

        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(long, short, required = false, help = "an optional tag name.")]
        tag: Option<String>,

        #[arg(
            long,
            short = 'w',
            required = false,
            help = "an optional associated website."
        )]
        site: Option<String>,
    },


    #[command(about = "search through your store.")]
    Search {
        #[arg(
            index = 1,
            help = "a required query.",
            long_help = "internally, it uses the starts-with filter so if you search with 'jh', it'll list you all credentials whose 'logins' start with 'jh'."
        )]
        query: String,

        #[arg(long, short, required = false, help = "an optional store name.")]
        store: Option<String>,

        #[arg(long, short, required = false)]
        filter: Option<String>,
    },

    #[command(about = "initialize 'fa' & create configuration.")]
    Init {
        #[arg(
            long,
            short,
            help = "a required gpg key fingerprint.",
            long_help = "this key will be used to encrypt/decrypt your stores. fingerprint is also known as key id sometimes. you can follow \"https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key\" till step 4 to know to get your key fingerprint."
        )]
        key_fingerprint: Option<String>,

        #[arg(
            long,
            short = 'n',
            help = "a required store name.",
            long_help = "this will be your default store name. this is upto you to choose!"
        )]
        store: Option<String>,

        #[arg(
            long,
            short = 'p',
            help = "a required store path",
            long_help = "this will be a directory where all the store's are located. i suggest creating a directory in a place you won't 'accidently' delete (lmao) and then doing 'git init'. then you can tell 'fa' about this directory via this flag and it'll save all the stores there."
        )]
        store_path: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum FaCommandConfig {
    #[command(about = "view the current set of configuration used by 'fa'.")]
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

    #[command(about = "create an empty store.")]
    Create {
        #[arg(index = 1, help = "a required store name.")]
        store: String,
    },

    #[command(about = "default a store.")]
    Default {
        #[arg(index = 1, help = "a required store name.")]
        store: String,
    },
}
