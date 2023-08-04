use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WikiError {
    #[error("A network error occurred")]
    Network(#[from] reqwest::Error),
    #[error("A yaml parsing error occurred")]
    YamlParsing(#[from] serde_yaml::Error),
    #[error("An IO error occurred")]
    IO(#[from] io::Error),
    #[error("A path error occurred")]
    Path(String),
    #[error("An HTML error occurred")]
    Html(String),
}
