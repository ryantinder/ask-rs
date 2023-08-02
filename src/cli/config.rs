use serde::{Serialize, Deserialize};
use crate::cli::err::*;
use crate::gpt::types::{Model, Prompt};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelConfig {
    pub default: usize,
    pub models: Vec<Model>
}

/// `AskConfig` implements `Default`
impl ::std::default::Default for ModelConfig {
    fn default() -> Self { Self { default: 0, models: vec![] } }
}


pub fn load_models() -> ModelConfig {
    confy::load("ask-rs", "models1").expect("error loading config")
}
pub fn store_models(cfg: ModelConfig) {
    confy::store("ask-rs", "models1", cfg).expect("error storing config");
    println!("Models Updated")
}


pub fn load_model() -> Result<Model, Error> {
    let cfg = load_models();
    match cfg.models.iter().nth(cfg.default).clone() {
        Some(m) => Ok(m.clone()),
        _ => Err(Error::NoModels)
    }    
}

pub fn store_model(model: Model) {
    let mut cfg = load_models();
    match cfg.models.iter_mut().find(|m| m.name == model.name) {
        Some(m) => {
            *m = model;
        },
        None => {
            cfg.models.push( model );
        }
    }
    store_models(cfg);
}

pub fn set_default_model(index: usize) {
    let mut cfg = load_models();
    cfg.default = index;
    store_models(cfg)
}

pub fn drop_model(index: usize) {
    let mut cfg = load_models();
    cfg.models.remove(index);
    if cfg.default == index {
        cfg.default = 0;
    }
    store_models(cfg)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptConfig {
    pub default: usize,
    pub prompts: Vec<Prompt>
}
/// `AskConfig` implements `Default`
impl ::std::default::Default for PromptConfig {
    fn default() -> Self { Self { default: 0, prompts: vec![] } }
}
pub fn load_prompts() -> PromptConfig {
    confy::load("ask-rs", "prompts").expect("error loading config")
}
pub fn store_prompts(cfg: PromptConfig) {
    confy::store("ask-rs", "prompts", cfg).expect("error storing config");
    println!("Prompts Updated")
}
pub fn load_prompt(name: Option<String>) -> Result<Prompt, Error> {
    let cfg = load_prompts();
    if let Some(name) = name {
        match cfg.prompts.iter().find(|e| e.name == name).clone() {
            Some(m) => Ok(m.clone()),
            _ => Err(Error::ModelNotFound { name })
        }
    } else {
        match cfg.prompts.iter().nth(cfg.default).clone() {
            Some(m) => Ok(m.clone()),
            _ => Err(Error::NoModels)
        }    
    }
}

pub fn store_prompt(prompt: Prompt) {
    let mut cfg = load_prompts();
    match cfg.prompts.iter_mut().find(|m| m.name == prompt.name) {
        Some(m) => {
            *m = prompt;
        },
        None => {
            cfg.prompts.push( prompt );
        }
    }
    store_prompts(cfg);
}

pub fn set_default_prompt(index: usize) {
    let mut cfg = load_prompts();
    cfg.default = index;
    store_prompts(cfg)
}

pub fn drop_prompt_i(index: usize) {
    let mut cfg = load_prompts();
    cfg.prompts.remove(index);
    if cfg.default > cfg.prompts.len() {
        cfg.default = 0;
    }
    store_prompts(cfg)
}
// pub fn load_all_prompts() -> Vec<Prompt> {
//     let cfg = load_config();
//     cfg.profiles
// }

// pub fn load_prompt(name: String) -> Result<Prompt, Error> {
//     let prompts = load_all_prompts();
//     let prompt = prompts.iter().find(|e| e.name == name).clone();
//     match prompt {
//         Some(p) => Ok(p.clone()),
//         _ => Err(Error::ProfileNotFound { name })
//     }
// }