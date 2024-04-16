use std::{fmt, io, time::SystemTimeError};

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum WikiError {
    #[error("A network error occurred.\nERROR: {}", .0)]
    Network(#[from] reqwest::Error),
    #[error("A yaml parsing/serialization error occurred.\nERROR: {}", .0)]
    YamlParsing(#[from] serde_yaml::Error),
    #[error("A json parsing/serialization error occurred.\nERROR: {}", .0)]
    JsonParsing(#[from] serde_json::Error),
    #[error("A URL parsing error occured.\nERROR: {}", .0)]
    UrlParseError(#[from] url::ParseError),
    #[error("An invalid api response was received.\nERROR: {}", .0)]
    InvalidApiResponse(InvalidApiResponse),
    #[error("{}", .0)]
    NoPageFound(String),
    #[cfg(feature = "cli")]
    #[error("An IO error occurred.\nERROR: {}", .0)]
    IO(#[from] io::Error),
    #[cfg(feature = "cli")]
    #[error("A path error occurred.\nERROR: {}", .0)]
    Path(String),
    #[cfg(feature = "cli")]
    #[error("A system time error occurred.\nERROR: {}", .0)]
    SystemTime(#[from] SystemTimeError),
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
#[derive(Debug)]
#[wasm_bindgen::prelude::wasm_bindgen]
#[allow(clippy::module_name_repetitions)]
pub struct WasmWikiError {
    kind: WasmWikiErrorKind,
    error: String,
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
#[wasm_bindgen::prelude::wasm_bindgen]
impl WasmWikiError {
    #[wasm_bindgen::prelude::wasm_bindgen(getter)]
    pub fn kind(&self) -> WasmWikiErrorKind {
        self.kind.clone()
    }

    #[wasm_bindgen::prelude::wasm_bindgen(getter)]
    pub fn error(&self) -> String {
        self.error.clone()
    }
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
impl From<WikiError> for WasmWikiError {
    fn from(value: WikiError) -> Self {
        Self {
            kind: WasmWikiErrorKind::from(&value),
            error: value.to_string(),
        }
    }
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug, Clone)]
pub enum WasmWikiErrorKind {
    Network,
    YamlParsing,
    JsonParsing,
    JsValueParsing,
    UrlParseError,
    InvalidApiResponse,
    NoPageFound,
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
impl<'a> From<&'a WikiError> for WasmWikiErrorKind {
    fn from(value: &'a WikiError) -> Self {
        match value {
            WikiError::Network(_) => Self::Network,
            WikiError::YamlParsing(_) => Self::YamlParsing,
            WikiError::JsonParsing(_) => Self::JsonParsing,
            WikiError::UrlParseError(_) => Self::UrlParseError,
            WikiError::InvalidApiResponse(_) => Self::InvalidApiResponse,
            WikiError::NoPageFound(_) => Self::NoPageFound,
        }
    }
}

#[cfg(all(
    not(feature = "cli"),
    any(feature = "wasm-web", feature = "wasm-nodejs"),
))]
impl From<serde_wasm_bindgen::Error> for WasmWikiError {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        Self {
            error: value.to_string(),
            kind: WasmWikiErrorKind::JsValueParsing,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum InvalidApiResponse {
    OpenSearchMissingNthElement(usize),
    OpenSearchNthElementShouldBeArray(usize),
    OpenSearchArraysLengthMismatch,
}

impl fmt::Display for InvalidApiResponse {
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
