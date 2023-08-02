use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    pub question: Option<Vec<String>>,
    #[arg(short, long)]
    pub profile: Option<String>,
    #[clap(subcommand)]
    pub opts: Option<Subcommands>
}
#[derive(Debug, Subcommand)]
pub enum Subcommands {
    Prompt(PromptsArgs),
    Key(KeyArgs)
}
#[derive(Parser, Debug)]
pub struct PromptsArgs {
    #[clap(subcommand)]
    pub cmd: PromptSubcommands
}
#[derive(Debug, Subcommand)]
pub enum PromptSubcommands {
    New,
    All,
    Edit {
        name: String
    },
    Switch {
        name: String
    },
    Drop {
        name: String
    }
}
#[derive(Parser, Debug)]
pub struct KeyArgs {
    #[clap(subcommand)]
    pub cmd: KeySubcommands
}
#[derive(Debug, Subcommand)]
pub enum KeySubcommands {
    New,
    All,
    Switch {
        name: String
    },
    Drop {
        name: String
    }
}
