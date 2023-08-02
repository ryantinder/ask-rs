use dialoguer::{Select, Password, Input, Editor, };
use rustyline::{DefaultEditor, Result, error::ReadlineError};

pub fn select_from<T>(items: &[T]) -> Option<usize> 
where 
    T: std::fmt::Display 
{
    let chosen = Select::new()
        .items(&items)
        .with_prompt("Select model")
        .default(0)
        .interact_opt();
    match chosen {
        Err(_) => None,
        Ok(choice) => {
            match choice {
                Some(c) => Some(c),
                None => None
            }
        }
    }
}

pub fn input_key() -> Option<String> {
    let password = Password::new().with_prompt("API Key")
        .interact();

    match password {
        Ok(str) => Some(str),
        Err(_) => None
    }
}
pub fn input_id() -> String {
    let input = Input::new().with_prompt("Model identifier")
        .interact();
    match input {
        Ok(str) => if String::is_empty( &str ) { String::from("default") } else { str } ,
        Err(_) => String::from("default")
    }
}
pub fn input_name() -> String {
    let input = Input::new().with_prompt("Prompt name")
        .interact();
    match input {
        Ok(str) => if String::is_empty( &str ) { String::from("default") } else { str } ,
        Err(_) => String::from("default")
    }
}
pub fn input_prompt(content: String) -> Option<String> {
    let mut rl = DefaultEditor::new().unwrap();
    let input = rl.readline_with_initial("Edit prompt: ", (content.as_str(), ""));
    match input {
        Ok(line) => {
            Some(line)
        },
        _ => Some(content)
    }
}