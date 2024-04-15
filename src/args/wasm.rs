#![cfg(any(feature = "wasm-nodejs", feature = "wasm-web"))]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::wasm_bindgen;

use crate::formats::PageFormat;

use super::internal::{
    InfoArgs, InfoPlainArgs, ListCategoriesArgs, ListCategoriesFmtArgs, ListLanguagesArgs,
    ListLanguagesFmtArgs, ListPagesArgs, ListPagesFmtArgs, ListPagesPlainArgs, ReadPageArgs,
    SearchArgs, SearchFmtArgs, WikiMetadataArgs, WikiMetadataFmtArgs,
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
            None | Some(SearchFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(SearchFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            Some(SearchFmtWasmArgs::Plain) => Self::Plain,
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
            None | Some(WikiMetadataFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(WikiMetadataFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            Some(WikiMetadataFmtWasmArgs::Yaml) => Self::Yaml,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ListPagesWasmArgs {
    fmt: Option<ListPagesFmtWasmArgs>,
    args_plain: Option<ListPagesPlainWasmArgs>,
}

#[wasm_bindgen]
impl ListPagesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        fmt: Option<ListPagesFmtWasmArgs>,
        argsPlain: Option<ListPagesPlainWasmArgs>,
    ) -> Self {
        Self {
            fmt,
            args_plain: argsPlain,
        }
    }
}

impl From<ListPagesWasmArgs> for ListPagesArgs {
    fn from(ListPagesWasmArgs { fmt, args_plain }: ListPagesWasmArgs) -> Self {
        Self {
            fmt: fmt.into(),
            args_plain: args_plain.map(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListPagesFmtWasmArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListPagesFmtWasmArgs>> for ListPagesFmtArgs {
    fn from(value: Option<ListPagesFmtWasmArgs>) -> Self {
        match value {
            None | Some(ListPagesFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(ListPagesFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            Some(ListPagesFmtWasmArgs::Plain) => Self::Plain,
        }
    }
}
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ListPagesPlainWasmArgs {
    flatten: bool,
    categories: Vec<String>,
}

impl From<ListPagesPlainWasmArgs> for ListPagesPlainArgs {
    fn from(
        ListPagesPlainWasmArgs {
            flatten,
            categories,
        }: ListPagesPlainWasmArgs,
    ) -> Self {
        Self {
            flatten,
            categories,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesWasmArgs {
    fmt: Option<ListCategoriesFmtWasmArgs>,
}

#[wasm_bindgen]
impl ListCategoriesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<ListCategoriesFmtWasmArgs>) -> Self {
        Self { fmt }
    }
}

impl From<ListCategoriesWasmArgs> for ListCategoriesArgs {
    fn from(ListCategoriesWasmArgs { fmt }: ListCategoriesWasmArgs) -> Self {
        Self { fmt: fmt.into() }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListCategoriesFmtWasmArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListCategoriesFmtWasmArgs>> for ListCategoriesFmtArgs {
    fn from(value: Option<ListCategoriesFmtWasmArgs>) -> Self {
        match value {
            None | Some(ListCategoriesFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(ListCategoriesFmtWasmArgs::JsonPretty) => Self::JsonPretty,
            Some(ListCategoriesFmtWasmArgs::Plain) => Self::Plain,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesWasmArgs {
    fmt: Option<ListLanguagesFmtWasmArgs>,
}

#[wasm_bindgen]
impl ListLanguagesWasmArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<ListLanguagesFmtWasmArgs>) -> Self {
        Self { fmt }
    }
}

impl From<ListLanguagesWasmArgs> for ListLanguagesArgs {
    fn from(ListLanguagesWasmArgs { fmt }: ListLanguagesWasmArgs) -> Self {
        Self { fmt: fmt.into() }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListLanguagesFmtWasmArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListLanguagesFmtWasmArgs>> for ListLanguagesFmtArgs {
    fn from(value: Option<ListLanguagesFmtWasmArgs>) -> Self {
        match value {
            None | Some(ListLanguagesFmtWasmArgs::Plain) => Self::Plain,
            Some(ListLanguagesFmtWasmArgs::JsonRaw) => Self::JsonRaw,
            Some(ListLanguagesFmtWasmArgs::JsonPretty) => Self::JsonPretty,
        }
    }
}
