#![cfg(any(feature = "wasm-nodejs", feature = "wasm-web"))]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::wasm_bindgen;

use crate::formats::PageFormat;

use super::internal;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ReadPageArgs {
    page: String,
    format: Option<PageFormat>,
    lang: Option<String>,
    show_urls: Option<bool>,
}

#[wasm_bindgen]
impl ReadPageArgs {
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

impl From<ReadPageArgs> for internal::ReadPageArgs {
    fn from(
        ReadPageArgs {
            page,
            format,
            lang,
            show_urls,
        }: ReadPageArgs,
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
pub struct SearchArgs {
    search: String,
    lang: Option<String>,
    limit: Option<u16>,
    text_search: Option<bool>,
    fmt: Option<SearchFmtArgs>,
}

#[wasm_bindgen]
impl SearchArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        search: String,
        lang: Option<String>,
        limit: Option<u16>,
        textSearch: Option<bool>,
        fmt: Option<SearchFmtArgs>,
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

impl From<SearchArgs> for internal::SearchArgs {
    fn from(
        SearchArgs {
            search,
            lang,
            limit,
            text_search,
            fmt,
        }: SearchArgs,
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
pub enum SearchFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<SearchFmtArgs>> for internal::SearchFmtArgs {
    fn from(value: Option<SearchFmtArgs>) -> Self {
        match value {
            None | Some(SearchFmtArgs::JsonRaw) => Self::JsonRaw,
            Some(SearchFmtArgs::JsonPretty) => Self::JsonPretty,
            Some(SearchFmtArgs::Plain) => Self::Plain,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct WikiMetadataArgs {
    fmt: Option<WikiMetadataFmtArgs>,
}

#[wasm_bindgen]
impl WikiMetadataArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<WikiMetadataFmtArgs>) -> Self {
        Self { fmt }
    }
}

impl From<WikiMetadataArgs> for internal::WikiMetadataArgs {
    fn from(WikiMetadataArgs { fmt }: WikiMetadataArgs) -> Self {
        Self {
            hide_progress: true,
            fmt: fmt.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum WikiMetadataFmtArgs {
    JsonPretty,
    JsonRaw,
    Yaml,
}

impl From<Option<WikiMetadataFmtArgs>> for internal::WikiMetadataFmtArgs {
    fn from(value: Option<WikiMetadataFmtArgs>) -> Self {
        match value {
            None | Some(WikiMetadataFmtArgs::JsonRaw) => Self::JsonRaw,
            Some(WikiMetadataFmtArgs::JsonPretty) => Self::JsonPretty,
            Some(WikiMetadataFmtArgs::Yaml) => Self::Yaml,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ListPagesArgs {
    fmt: Option<ListPagesFmtArgs>,
    args_plain: Option<ListPagesPlainArgs>,
}

#[wasm_bindgen]
impl ListPagesArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<ListPagesFmtArgs>, argsPlain: Option<ListPagesPlainArgs>) -> Self {
        Self {
            fmt,
            args_plain: argsPlain,
        }
    }
}

impl From<ListPagesArgs> for internal::ListPagesArgs {
    fn from(ListPagesArgs { fmt, args_plain }: ListPagesArgs) -> Self {
        Self {
            fmt: fmt.into(),
            args_plain: args_plain.map(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListPagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListPagesFmtArgs>> for internal::ListPagesFmtArgs {
    fn from(value: Option<ListPagesFmtArgs>) -> Self {
        match value {
            None | Some(ListPagesFmtArgs::JsonRaw) => Self::JsonRaw,
            Some(ListPagesFmtArgs::JsonPretty) => Self::JsonPretty,
            Some(ListPagesFmtArgs::Plain) => Self::Plain,
        }
    }
}
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ListPagesPlainArgs {
    flatten: bool,
    categories: Vec<String>,
}

impl From<ListPagesPlainArgs> for internal::ListPagesPlainArgs {
    fn from(
        ListPagesPlainArgs {
            flatten,
            categories,
        }: ListPagesPlainArgs,
    ) -> Self {
        Self {
            flatten,
            categories,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListCategoriesArgs {
    fmt: Option<ListCategoriesFmtArgs>,
}

#[wasm_bindgen]
impl ListCategoriesArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<ListCategoriesFmtArgs>) -> Self {
        Self { fmt }
    }
}

impl From<ListCategoriesArgs> for internal::ListCategoriesArgs {
    fn from(ListCategoriesArgs { fmt }: ListCategoriesArgs) -> Self {
        Self { fmt: fmt.into() }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListCategoriesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListCategoriesFmtArgs>> for internal::ListCategoriesFmtArgs {
    fn from(value: Option<ListCategoriesFmtArgs>) -> Self {
        match value {
            None | Some(ListCategoriesFmtArgs::JsonRaw) => Self::JsonRaw,
            Some(ListCategoriesFmtArgs::JsonPretty) => Self::JsonPretty,
            Some(ListCategoriesFmtArgs::Plain) => Self::Plain,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct ListLanguagesArgs {
    fmt: Option<ListLanguagesFmtArgs>,
}

#[wasm_bindgen]
impl ListLanguagesArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(fmt: Option<ListLanguagesFmtArgs>) -> Self {
        Self { fmt }
    }
}

impl From<ListLanguagesArgs> for internal::ListLanguagesArgs {
    fn from(ListLanguagesArgs { fmt }: ListLanguagesArgs) -> Self {
        Self { fmt: fmt.into() }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ListLanguagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

impl From<Option<ListLanguagesFmtArgs>> for internal::ListLanguagesFmtArgs {
    fn from(value: Option<ListLanguagesFmtArgs>) -> Self {
        match value {
            None | Some(ListLanguagesFmtArgs::Plain) => Self::Plain,
            Some(ListLanguagesFmtArgs::JsonRaw) => Self::JsonRaw,
            Some(ListLanguagesFmtArgs::JsonPretty) => Self::JsonPretty,
        }
    }
}
