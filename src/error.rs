use std::{io, time::SystemTimeError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WikiError {
    #[error("A network error occurred.\nERROR: {}", .0)]
    Network(#[from] reqwest::Error),
    #[error("A yaml parsing error occurred.\nERROR: {}", .0)]
    YamlParsing(#[from] serde_yaml::Error),
    #[error("A json parsing error occurred.\nERROR: {}", .0)]
    JsonParsing(#[from] serde_json::Error),
    #[error("An IO error occurred.\nERROR: {}", .0)]
    IO(#[from] io::Error),
    #[error("A system time error occurred.\nERROR: {}", .0)]
    SystemTime(#[from] SystemTimeError),
    #[error("A path error occurred.\nERROR: {}", .0)]
    Path(String),
    #[error("A HTML error occurred.\nERROR: {}", .0)]
    Html(String),
    #[error("{}", .0)]
    NoPageFound(String),
}
