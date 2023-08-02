use clap::{Parser, Subcommand};
use anyhow::{Result};
use cli::config::{load_model, load_prompt};
use cli::prompt::{new_prompt, switch_prompt, drop_prompt, all_prompts, edit_prompt, self};
use futures_util::StreamExt;
use std::io::*;

mod cli;
mod gpt;

use gpt::client::Client;
use gpt::types::ResponseChunk;
use cli::types::{Cli, Subcommands, KeySubcommands, PromptSubcommands};
use cli::key::{new_key, all_keys, switch_key, drop_key};
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    if let Some(q_arr) = args.question {
        if let Ok(model) = load_model() {
            let question = q_arr.join(" ");
            let mut client = Client::new(model)?;

            if let Ok(prompt) = load_prompt(args.profile.clone()) {
                println!("<including prompt {}>", args.profile.unwrap());
                client.add_prompt(prompt)
            }

            let mut stream = client.post_streaming(question.clone()).await?;
            while let Some(piece) = stream.next().await {
                match piece {
                    ResponseChunk::Content {
                        delta,
                        response_index : _
                    } => {
                        print!("{delta}");
                        stdout().lock().flush().unwrap();
                    },
                    _ => {}
                }
            }
        } else {
            println!("Error loading api key")
        }


    } 
    if let Some(opts) = args.opts {
        match opts {
            Subcommands::Key( opts ) => {
                match opts.cmd {
                    KeySubcommands::New => {
                        new_key();
                    },
                    KeySubcommands::All => {
                        all_keys();
                    },
                    KeySubcommands::Switch { name } => {
                        switch_key(name)
                    },
                    KeySubcommands::Drop { name } => {
                        drop_key(name)
                    }
                }
            },
            Subcommands::Prompt( opts ) => {
                match opts.cmd {
                    PromptSubcommands::New => {
                        new_prompt()
                    }
                    PromptSubcommands::All => {
                        all_prompts()
                    }
                    PromptSubcommands::Edit { name } => {
                        edit_prompt(name)
                    }
                    PromptSubcommands::Switch { name } => {
                        switch_prompt(name)
                    }
                    PromptSubcommands::Drop { name } => {
                        drop_prompt(name)
                    }
                }
            }
        }
    }
    Ok(())
}
