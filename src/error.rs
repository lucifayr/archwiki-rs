use std::{fmt, io, time::SystemTimeError};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum InvalidApiResponseError {
    OpenSearchMissingNthElement(usize),
    OpenSearchNthElementShouldBeArray(usize),
    OpenSearchArraysLengthMismatch,
}

impl fmt::Display for InvalidApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::OpenSearchMissingNthElement(n) => {
                format!("missing element #{n} in open search response")
            }
            Self::OpenSearchNthElementShouldBeArray(n) => {
                format!("expected element #{n} in open search response to be an array")
            }
            Self::OpenSearchArraysLengthMismatch => {
                "arrays in open search response should have the same length but do not".to_owned()
            }
        };

        write!(f, "{str}")
    }
}

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
    #[error("A URL parse error occured.\nERROR: {}", .0)]
    UrlParseError(#[from] url::ParseError),
    #[error("An invalid api response was received.\nERROR: {}", .0)]
    InvalidApiResponse(InvalidApiResponseError),
    #[error("{}", .0)]
    NoPageFound(String),
}
