use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "itree",
    version,
    about = "Inventree command line interface",
    long_about = r#"
  __  ___________  _______    _______   _______
 |" \("     _   ")/"      \  /"     "| /"     "|
 ||  |)__/  \\__/|:        |(: ______)(: ______)
 |:  |   \\_ /   |_____/   ) \/    |   \/    |
 |.  |   |.  |    //      /  // ___)_  // ___)_
 /\  |\  \:  |   |:  __   \ (:      "|(:      "|
(__\_|_)  \__|   |__|  \___) \_______) \_______)

itree is the central CLI for interacting with the inventree inventory managmement system.
"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ItreeCommands,

    /// Override the configured server URL
    #[arg(long, short = 'u', global = true, env = "ITREE_SERVER_URL")]
    pub url: Option<String>,

    /// Override the configured API token
    #[arg(long, short = 't', global = true, env = "ITREE_API_TOKEN")]
    pub token: Option<String>,

    #[command(flatten)]
    pub output: OutputFlags,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

#[derive(Parser, Clone)]
pub struct OutputFlags {
    /// Output format
    #[arg(long, short = 'o', global = true, default_value = "table")]
    pub output: OutputFormat,

    /// Show request/response details
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Max results for list commands
    #[arg(long, global = true)]
    pub limit: Option<u32>,
}

#[derive(Subcommand)]
pub enum ItreeCommands {
    /// Config operations
    Config {
        #[command(subcommand)]
        command: ItreeConfigCommands,
    },
    /// Category operations
    Category {
        #[command(subcommand)]
        command: ItreeCategoryCommands,
    },
    /// Part operations
    Part {
        #[command(subcommand)]
        command: ItreePartCommands,
    },
    /// Stock operations
    Stock {
        #[command(subcommand)]
        command: ItreeStockCommands,
    },
    /// Location operations
    Location {
        #[command(subcommand)]
        command: ItreeLocationCommands,
    }
}

#[derive(Subcommand)]
pub enum ItreeConfigCommands {
    /// Print the full config
    List,

    /// Get a config value: itree config get <header> <key>
    /// e.g. itree config get server url
    Get {
        header: String,
        key: String,
    },

    /// Set a config value: itree config set <header> <key> <value>
    /// e.g. itree config set server url http://iventree.localhost
    Set {
        header: String,
        key: String,
        value: String,
    },
}

#[derive(Subcommand)]
pub enum ItreeCategoryCommands {
    List,
    Get,
    Add,
    Remove
}

#[derive(Subcommand)]
pub enum ItreePartCommands {
    List,
    Get,
    Add,
    Remove
}


#[derive(Subcommand)]
pub enum ItreeStockCommands {
    List,
    Get,
    Add,
    Remove
}

#[derive(Subcommand)]
pub enum ItreeLocationCommands {
    List,
    Get,
    Add,
    Remove
}
