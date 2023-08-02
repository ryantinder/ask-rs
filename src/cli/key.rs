use super::{editor::{select_from, input_key, input_id}, config::{load_models, set_default_model, drop_model}};
use crate::gpt::types::{ChatGPTEngine, Model};
use super::config::{store_model};

pub fn new_key() {
    let all_models = [
        ChatGPTEngine::Gpt35Turbo,
        ChatGPTEngine::Gpt35Turbo_0301,
        ChatGPTEngine::Gpt4,
        ChatGPTEngine::Gpt4_32k
    ];
    // obtain model we want to add a key for
    let index = select_from(&all_models);
    if let None = index {
        println!("No model selected");
        return;
    }
    // obtain key
    let key = input_key();
    if let None = key {
        println!("No key given");
        return;
    }

    // obtain model id
    let id = input_id();
    store_model( Model {
        name: id.clone(), 
        api_key: key.unwrap().clone(), 
        model: all_models[index.unwrap()].to_string()
    })
    // store into config
}

pub fn all_keys() {
    let keys = load_models();
    keys.models.iter().for_each(|k| {
        println!("{}", k)
    });
}
pub fn switch_key(name: String) {
    match load_models().models.iter().position(|e| e.name == name) {
        Some(i) => set_default_model(i),
        None => {
            println!("Model not found, here are the options");
            all_keys();
        }
    }
}

pub fn drop_key(name: String) {
    match load_models().models.iter().position(|e| e.name == name) {
        Some(i) => drop_model(i),
        None => {
            println!("Model not found, here are the options");
            all_keys();
        }
    }
}