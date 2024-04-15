#![cfg(any(feature = "wasm-nodejs", feature = "wasm-web"))]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::wasm_bindgen;

use crate::formats::PageFormat;

use super::internal::{
    InfoArgs, InfoJsonArgs, InfoPlainArgs, ListCategoriesArgs, ListCategoriesJsonArgs,
    ListCategoriesPlainArgs, ListLanguagesArgs, ListLanguagesJsonArgs, ListLanguagesPlainArgs,
    ListPagesArgs, ListPagesJsonArgs, ListPagesPlainArgs, ReadPageArgs, SearchArgs, SearchFmtArgs,
    WikiMetadataArgs, WikiMetadataFmtArgs,
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ReadPageWasmArgs {
    page: String,
    format: Option<PageFormat>,
    lang: Option<String>,
    show_urls: Option<bool>,
}

#[wasm_bindgen]
impl ReadPageWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        page: String,
        format: Option<PageFormat>,
        lang: Option<String>,
        showUrls: Option<bool>,
    ) -> Self {
        Self {
            page,
            format,
            lang,
            show_urls: showUrls,
        }
    }
}

impl From<ReadPageWasmArgs> for ReadPageArgs {
    fn from(
        ReadPageWasmArgs {
            page,
            format,
            lang,
            show_urls,
        }: ReadPageWasmArgs,
    ) -> Self {
        Self {
            page,
            format: format.unwrap_or(PageFormat::Html),
            lang: lang.unwrap_or_else(|| Self::default().lang),
            show_urls: show_urls.unwrap_or_else(|| Self::default().show_urls),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct SearchWasmArgs {
    search: String,
    lang: Option<String>,
    limit: Option<u16>,
    text_search: Option<bool>,
    fmt: Option<SearchFmtWasmArgs>,
}

#[wasm_bindgen]
impl SearchWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        search: String,
        lang: Option<String>,
        limit: Option<u16>,
        textSearch: Option<bool>,
        fmt: Option<SearchFmtWasmArgs>,
    ) -> Self {
        Self {
            search,
            lang,
            limit,
            fmt,
            text_search: textSearch,
        }
    }
}

impl From<SearchWasmArgs> for SearchArgs {
    fn from(
        SearchWasmArgs {
            search,
            lang,
            limit,
            text_search,
            fmt,
        }: SearchWasmArgs,
    ) -> Self {
        Self {
            search,
            lang: lang.unwrap_or_else(|| Self::default().lang),
            limit: limit.unwrap_or_else(|| Self::default().limit),
            text_search: text_search.unwrap_or_else(|| Self::default().text_search),
            fmt: fmt.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum SearchFmtWasmArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<SearchFmtWasmArgs>> for SearchFmtArgs {
    fn from(value: Option<SearchFmtWasmArgs>) -> Self {
        match value {
            Some(SearchFmtWasmArgs::Plain) => Self::Plain,
            Some(SearchFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(SearchFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            None => Self::JsonRaw,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WikiMetadataWasmArgs {
    fmt: Option<WikiMetadataFmtWasmArgs>,
}

#[wasm_bindgen]
impl WikiMetadataWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<WikiMetadataFmtWasmArgs>) -> Self {
        Self { fmt }
    }
}

impl From<WikiMetadataWasmArgs> for WikiMetadataArgs {
    fn from(WikiMetadataWasmArgs { fmt }: WikiMetadataWasmArgs) -> Self {
        Self {
            hide_progress: true,
            fmt: fmt.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum WikiMetadataFmtWasmArgs {
    JsonPretty,
    JsonRaw,
    Yaml,
}

impl From<Option<WikiMetadataFmtWasmArgs>> for WikiMetadataFmtArgs {
    fn from(value: Option<WikiMetadataFmtWasmArgs>) -> Self {
        match value {
            Some(WikiMetadataFmtWasmArgs::Yaml) => Self::Yaml,
            Some(WikiMetadataFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(WikiMetadataFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            None => Self::JsonRaw,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesWasmArgs {
    args_plain: Option<ListPagesPlainWasmArgs>,
    args_json: Option<ListPagesJsonWasmArgs>,
}

#[wasm_bindgen]
impl ListPagesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        argsPlain: Option<ListPagesPlainWasmArgs>,
        argsJson: Option<ListPagesJsonWasmArgs>,
    ) -> Self {
        Self {
            args_plain: argsPlain,
            args_json: argsJson,
        }
    }
}

impl From<ListPagesWasmArgs> for ListPagesArgs {
    fn from(
        ListPagesWasmArgs {
            args_plain,
            args_json,
        }: ListPagesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesPlainWasmArgs {
    flatten: Option<bool>,
    categories: Option<Vec<String>>,
}

#[wasm_bindgen]
impl ListPagesPlainWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(flatten: Option<bool>, categories: Option<Vec<String>>) -> Self {
        Self {
            flatten,
            categories,
        }
    }
}

impl From<ListPagesPlainWasmArgs> for ListPagesPlainArgs {
    fn from(
        ListPagesPlainWasmArgs {
            flatten,
            categories,
        }: ListPagesPlainWasmArgs,
    ) -> Self {
        Self {
            flatten: flatten.unwrap_or_else(|| Self::default().flatten),
            categories: categories.unwrap_or_else(|| Self::default().categories),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListPagesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

#[wasm_bindgen]
impl ListPagesJsonWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(json: Option<bool>, jsonRaw: Option<bool>) -> Self {
        Self {
            json,
            json_raw: jsonRaw,
        }
    }
}

impl Default for ListPagesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListPagesJsonWasmArgs> for ListPagesJsonArgs {
    fn from(ListPagesJsonWasmArgs { json, json_raw }: ListPagesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesWasmArgs {
    args_plain: Option<ListCategoriesPlainWasmArgs>,
    args_json: Option<ListCategoriesJsonWasmArgs>,
}

#[wasm_bindgen]
impl ListCategoriesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        argsPlain: Option<ListCategoriesPlainWasmArgs>,
        argsJson: Option<ListCategoriesJsonWasmArgs>,
    ) -> Self {
        Self {
            args_plain: argsPlain,
            args_json: argsJson,
        }
    }
}

impl From<ListCategoriesWasmArgs> for ListCategoriesArgs {
    fn from(
        ListCategoriesWasmArgs {
            args_json,
            args_plain,
        }: ListCategoriesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}
#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesPlainWasmArgs {
    plain: Option<bool>,
}

#[wasm_bindgen]
impl ListCategoriesPlainWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(plain: Option<bool>) -> Self {
        Self { plain }
    }
}

impl From<ListCategoriesPlainWasmArgs> for ListCategoriesPlainArgs {
    fn from(ListCategoriesPlainWasmArgs { plain }: ListCategoriesPlainWasmArgs) -> Self {
        Self {
            plain: plain.unwrap_or_else(|| Self::default().plain),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

#[wasm_bindgen]
impl ListCategoriesJsonWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(json: Option<bool>, jsonRaw: Option<bool>) -> Self {
        Self {
            json,
            json_raw: jsonRaw,
        }
    }
}

impl Default for ListCategoriesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListCategoriesJsonWasmArgs> for ListCategoriesJsonArgs {
    fn from(ListCategoriesJsonWasmArgs { json, json_raw }: ListCategoriesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesWasmArgs {
    args_plain: Option<ListLanguagesPlainWasmArgs>,
    args_json: Option<ListLanguagesJsonWasmArgs>,
}

#[wasm_bindgen]
impl ListLanguagesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        argsPlain: Option<ListLanguagesPlainWasmArgs>,
        argsJson: Option<ListLanguagesJsonWasmArgs>,
    ) -> Self {
        Self {
            args_plain: argsPlain,
            args_json: argsJson,
        }
    }
}

impl From<ListLanguagesWasmArgs> for ListLanguagesArgs {
    fn from(
        ListLanguagesWasmArgs {
            args_json,
            args_plain,
        }: ListLanguagesWasmArgs,
    ) -> Self {
        Self {
            args_plain: args_plain.map(Into::into),
            args_json: Some(args_json.unwrap_or_default().into()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesPlainWasmArgs {
    plain: Option<bool>,
}

#[wasm_bindgen]
impl ListLanguagesPlainWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(plain: Option<bool>) -> Self {
        Self { plain }
    }
}

impl From<ListLanguagesPlainWasmArgs> for ListLanguagesPlainArgs {
    fn from(ListLanguagesPlainWasmArgs { plain }: ListLanguagesPlainWasmArgs) -> Self {
        Self {
            plain: plain.unwrap_or_else(|| Self::default().plain),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesJsonWasmArgs {
    json: Option<bool>,
    json_raw: Option<bool>,
}

#[wasm_bindgen]
impl ListLanguagesJsonWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(json: Option<bool>, jsonRaw: Option<bool>) -> Self {
        Self {
            json,
            json_raw: jsonRaw,
        }
    }
}

impl Default for ListLanguagesJsonWasmArgs {
    fn default() -> Self {
        Self {
            json: None,
            json_raw: Some(true),
        }
    }
}

impl From<ListLanguagesJsonWasmArgs> for ListLanguagesJsonArgs {
    fn from(ListLanguagesJsonWasmArgs { json, json_raw }: ListLanguagesJsonWasmArgs) -> Self {
        Self {
            json: json.unwrap_or_else(|| Self::default().json),
            json_raw: json_raw.unwrap_or_else(|| Self::default().json_raw),
        }
    }
}
