use crate::formats::PageFormat;

#[derive(Debug, Clone)]
pub struct ReadPageArgs {
    pub page: String,
    pub format: PageFormat,
    pub lang: String,
    pub show_urls: bool,
}

impl Default for ReadPageArgs {
    fn default() -> Self {
        Self {
            page: String::default(),
            format: PageFormat::default(),
            lang: String::from("en"),
            show_urls: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchArgs {
    pub search: String,
    pub lang: String,
    pub limit: u16,
    pub text_search: bool,
    pub fmt: SearchFmtArgs,
}

impl Default for SearchArgs {
    fn default() -> Self {
        Self {
            search: String::default(),
            lang: String::from("en"),
            limit: 5,
            text_search: false,
            fmt: SearchFmtArgs::Plain,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct WikiMetadataArgs {
    pub hide_progress: bool,
    pub fmt: WikiMetadataFmtArgs,
}

#[derive(Debug, Clone)]
pub enum WikiMetadataFmtArgs {
    JsonPretty,
    JsonRaw,
    Yaml,
}

#[derive(Debug, Clone)]
pub struct ListPagesArgs {
    pub fmt: ListPagesFmtArgs,
    pub args_plain: Option<ListPagesPlainArgs>,
}

#[derive(Debug, Clone, Default)]
pub struct ListPagesPlainArgs {
    pub flatten: bool,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ListPagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct ListCategoriesArgs {
    pub fmt: ListCategoriesFmtArgs,
}

#[derive(Debug, Clone)]
pub enum ListCategoriesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone)]
pub struct ListLanguagesArgs {
    pub fmt: ListLanguagesFmtArgs,
}

#[derive(Debug, Clone)]
pub enum ListLanguagesFmtArgs {
    JsonPretty,
    JsonRaw,
    Plain,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoArgs {
    pub args_plain: Option<InfoPlainArgs>,
    pub args_json: Option<InfoJsonArgs>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoPlainArgs {
    pub show_cache_dir: bool,
    pub show_data_dir: bool,
    pub only_values: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InfoJsonArgs {
    pub json: bool,
    pub json_raw: bool,
}
