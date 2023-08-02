use std::{env::VarError, string::FromUtf8Error};

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

/// An error enum, used in the Result
#[derive(Debug, Error)]
pub enum Error {
    #[error("Profile name {name} not found")]
    ProfileNotFound {
        /// Message, describing this error
        name: String
    },
    #[error("Modal name {name} not found")]
    ModelNotFound {
        /// Message, describing this error
        name: String
    },
    #[error("No models created yet")]
    NoModels,
}