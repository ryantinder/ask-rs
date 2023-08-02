use super::{editor::{input_id, input_prompt}, config::{load_prompts, set_default_prompt, drop_prompt_i, store_prompt, load_prompt}};
use crate::gpt::types::Prompt;

pub fn new_prompt() {
    // obtain model id
    let id = input_id();

    let content = input_prompt("".into());
    if let Some(c) = content {
        store_prompt( Prompt {
            name: id.clone(), 
            content: c
        })
    } else {
        println!("Prompt creation cancelled")
    }
}

pub fn all_prompts() {
    let cfg = load_prompts();
    cfg.prompts.iter().for_each(|k| {
        println!("{}", k)
    });
}
pub fn switch_prompt(name: String) {
    match load_prompts().prompts.iter().position(|e| e.name == name) {
        Some(i) => set_default_prompt(i),
        None => {
            println!("Prompt not found, here are the options");
            all_prompts();
        }
    }
}
pub fn edit_prompt(name: String) {
    match load_prompt(Some(name)) {
        Ok(prompt) => {
            let new_content = input_prompt(prompt.content.into());
            if let Some(c) = new_content {
                store_prompt( Prompt {
                    name: prompt.name.clone(), 
                    content: c
                })
            } else {
                println!("Prompt editing cancelled")
            }
        }
        Err(_) => {
            println!("Prompt not found, here are the options");
            all_prompts();
        }
    }
}

pub fn drop_prompt(name: String) {
    match load_prompts().prompts.iter().position(|e| e.name == name) {
        Some(i) => drop_prompt_i(i),
        None => {
            println!("Model not found, here are the options");
            all_prompts();
        }
    }
}